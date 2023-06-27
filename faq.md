## FAQ

#### What is PR?

PR is an abbreviation for [Pull Request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests).

#### When and how are the meetings held?

Meetings are held four times a week on weekdays (except Tuesday) according to your group's schedule. The meeting lasts about 1 hour. All appointments will be in your calendar. Attendance at the first meeting is mandatory.

#### Will the meeting be recorded?

No, there will be no recordings of the meetings, so do your best not to miss them.


#### How do I start completing tasks?

First, you need to create a private fork of the main [Rust Incubator repository](https://github.com/rust-lang-ua/rust_incubator). You can find more information on how to do this in the [instructions](https://github.com/rust-lang-ua/rust_incubator#before-you-start).

#### How and where do I send Pull Requests of completed tasks?

In your private fork, create a separate branch for each task. In the corresponding branch, complete the task and create a PR to your main master branch. Don't forget to tag the reviewers in your PR.

#### When is a task considered completed?

A task is considered completed when you've made a Pull Request, and it has been reviewed and approved by one of the reviewers.

#### What should I do if the task is unfinished? Should I open a PR?

Yes, you should open a PR as soon as possible and indicate "Not Ready" in the title.

#### Can I complete the course assignments only on weekends?

You should dedicate more than 20 hours a week to your studies to achieve quality results. In three months of intensive coursework, you can achieve what might otherwise take two years of independent practice. If you confine your study time to the weekends only, you would need to study for 10 hours a day. It's better to evenly distribute your study time throughout the week.

#### Where should I answer the questions in the first chapter?

You can leave the answers under the question itself by editing your own md-file.

#### Where can I ask for help?

Firstly, ask your peers in the Bootcamp's chat. You can also post your questions in the Telegram [chat of our community](https://t.me/rustlang_ua), the official [forum](https://users.rust-lang.org/). [ChatGPT](https://openai.com/blog/chatgpt) works well for simpler queries. However, your solutions must be original. Anyone found sharing or copying solutions will be expelled.

#### What is the best way to ask questions?

It's preferable to have your code in the [playground](https://play.rust-lang.org/). This way, mentors and peers don't have to spend extra time recreating the problem.

![playground_ask](https://github.com/rust-lang-ua/rust_incubator/assets/98274821/2351bddd-455f-4078-a7cb-328a7bb08ac9)

#### Is the Telegram chat moderated?

At the initial stage, we don't want to limit communication in the chat, nor do we want to divide it into sub-chats. We encourage people to get to know each other. After the familiarization phase, we may introduce stricter moderation rules in the future, if necessary. For now, however, you must respect other chat participants and are not allowed to be offensive.


#### What is the motivation of mentors to mentor?

All mentors are employed, and many of them have their own companies and are in a good position to hire a talented participant. Moreover, mentoring helps to improve their own knowledge, and all of our mentors have a natural desire to help other people.

#### Do we cooperate with IT companies in any way?

We are making a lot of efforts to involve companies in interviews with bootcamp graduates and hopefully we will get there. We currently have 4 partner companies: TacansLab, AI Edge Lab, Intellias, and Near.

#### What should I do if I find mistakes in the repository?
We encourage you to open an __issue__ if you see any problems.



#### How can I incorporate changes from the main repository?

Add the main repository as a remote.

```bash
# Add the remote
git remote add template git@github.com:rust-lang-ua/rust_incubator.git
# Fetch the changes from the repository
git fetch --all
# Merge the changes
git merge template/master
```

*If you get the error `fatal: refusing to merge unrelated histories` add option `--allow-unrelated-histories`  flag to the last command.*
