default:
    @just --list

# Fetch a spec section by id.
spec-section id *args:
    @scripts/spec_section.py "{{id}}" {{args}}

# Fetch a section's **Summary.** paragraph only.
spec-summary id:
    @scripts/spec_section.py "{{id}}" --summary

# Print the spec section index (full tree).
spec-index *args:
    @scripts/spec_index.py {{args}}

# Run spec integrity checks.
spec-verify *args:
    @scripts/spec_verify.py {{args}}

# Run spec integrity checks scoped to a specific section id.
spec-verify-scope id:
    @scripts/spec_verify.py "{{id}}"

# Generate a role-specific scaffold (Phase 7 stub).
spec-roles role:
    @scripts/spec_roles.py {{role}}
