# git-status-tree-rs
Rust respin of https://github.com/knugie/git-status-tree

## Why not exa?
`exa` has a git and tree mode: https://the.exa.website/features/git

However it's pretty limited; only supports *new* or *changed* files 😕 , and
doesn't allow filtering the response for un-checked-in changes only (so you end
up with a pile of results that you have to filter with ripgrep or whatnot).
