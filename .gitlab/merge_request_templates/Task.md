Resolves <paste issue reference>  
<and/or>  
Part of <paste issue/MR reference>  
<and/or>  
Required for <paste issues/MRs references>  
<and/or>  
Requires <paste issues/MRs references>  
<and/or>  
Related to <paste issues/MRs references>  

<Remove the lines above if there are no related issues/MRs>




## Synopsis

<Give a brief overview of the problem>




## Solution

<Describe how exactly the problem is (or will be) resolved>




## Checklist

- Created MR:
    - [ ] Name contains `WIP: ` prefix
    - [ ] Name contains issue reference
    - [ ] Has `type: ` and `kind: ` labels applied
- [ ] Documentation is updated (if required)
- [ ] Tests are updated (if required)
- [ ] Changes conform [code style][c:1]
- [ ] [CHANGELOG entry][c:2] is added (if required)
    - [ ] [Deployment instructions][c:3] are described (if required)
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
