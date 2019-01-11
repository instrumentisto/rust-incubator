Prepares [<paste release version>](<paste milestone link>) release




## Checklist

- Created MR:
    - [ ] Name contains `WIP: ` prefix
    - [ ] Name contains release version
    - [ ] Name contains milestone reference
    - [ ] Has `kind: ` labels applied
- [ ] Documentation is updated (if required)
- [ ] Tests are updated (if required)
- [ ] Changes conform [code style][c:1]
- [ ] [CHANGELOG entry][c:2] is verified and corrected
    - [ ] [Deployment instructions][c:3] are verified and corrected (if required)
- [ ] [FCM][c:4] is posted
    - [ ] and approved
- [ ] ~review is completed and changes are approved
- [ ] E2E tests pass (`test:e2e` manual CI job)
- Before merge:
    - [ ] Commits are [squashed and rebased][c:5]
    - [ ] Milestone is set
    - [ ] MR's name and description are correct and up-to-date
    - [ ] `WIP: ` prefix is removed
    - [ ] All temporary labels are removed





[c:1]: https://git.instrumentisto.com/common/documentation/blob/master/developers/codestyle.md
[c:2]: https://git.instrumentisto.com/common/documentation/blob/master/developers/workflow.md#changelog-1
[c:3]: https://git.instrumentisto.com/common/documentation/blob/master/developers/workflow.md#deployment-instructions
[c:4]: https://git.instrumentisto.com/common/documentation/blob/master/developers/workflow.md#fcm-final-commit-message
[c:5]: https://git.instrumentisto.com/common/documentation/blob/master/developers/workflow.md#merging




/assign me
/label ~"type: enhancement"
/label ~"kind: documentation"
