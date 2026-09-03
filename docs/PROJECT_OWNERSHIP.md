# Schematic project ownership

Konnect resolves saved schematic ownership by parsing candidate projects' sheet
trees. Directory ancestry alone does not establish ownership (#189). The shared
resolver serves symbol placement, library resolution, and ERC root detection.
Relative target paths are anchored at the process working directory before
walking ancestors, so the same saved hierarchy remains discoverable.

## Behavior

| Observed saved state | Result |
| --- | --- |
| Library table beside the target | Library lookup uses that directory before consulting ancestor projects. |
| Exactly one project hierarchy contains the target, and candidate traversal is complete | Use the proven project's directory and hierarchy paths. |
| No ancestor project candidate | Allow the loose schematic and keep its own directory as the library context. |
| Candidates exist but no hierarchy contains the target | Return the existing structured `conflict`. |
| A candidate root is missing, unreadable, malformed, or lacks its root UUID | Retain that candidate in the conflict evidence. |
| Multiple hierarchies contain the target | Return `conflict`; never choose by directory enumeration order. |
| A competing root or intermediate sheet cannot be inspected, or traversal reaches its depth limit | Return `conflict` even if another path to the target was found; incomplete observation does not prove uniqueness. |
| An ancestor directory cannot be enumerated | Return a `file_not_found` refusal naming that directory; do not report the schematic as projectless. |

Conflict `error.paths` contains the schematic directory and every candidate root
schematic, including roots that could not be read. Candidates are listed in
stable project-path order. A successful owner is derived from saved sheet references;
it is not inferred from the requested filename or a sibling project file.

Symbol-loading handlers resolve their library context before mutation. Batch
placement does so once before processing any entries, so ownership conflicts
cannot leave partially placed components. A local library table establishes
library authority, not proof of hierarchy membership for placement metadata.

## Unreleased notes (next minor release)

Schematics that previously inherited an unrelated ancestor project's libraries,
or silently fell back to projectless operation despite an unproven candidate,
now return `conflict`. Repair the saved root-to-child sheet references, restore
unreadable project schematics, or place a genuinely independent document outside
the unrelated project. An explicit library table beside a schematic continues
to select its library context. No tool, argument, or new error kind is added.

This behavior change belongs in the next minor release, as agreed in #189.
Existing `conflict` clients should inspect `error.paths` and the message before
retrying: reloading alone does not repair an ownership conflict.

## Evidence and limits

The `project_ownership` test fixture is copied from KiCad's complex-hierarchy
demo. Tests exercise its two references to one child, relocation into nested
directories, unrelated candidates, and explicit corruptions of temporary copies.

Resolution observes saved files, not unsaved editor state. Traversal is
cycle-safe and bounded by `MAX_HIERARCHY_DEPTH`; it does not establish ownership
through an untraversable hierarchy. This change does not implement the later
all-placement-instances work in #389 or live-editor navigation.

Ownership evidence is read during preflight; it is not an atomic snapshot of
every project file. Target-file writes retain their existing revision checks.
