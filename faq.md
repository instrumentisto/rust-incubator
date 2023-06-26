## FAQ

> PR - Pull Request

#### - When do the meetings are held?

4 times a week on weekdays according to your group's schedule.

#### - How do I start completing tasks?

First, you need to make a private fork of the main [Rust Incubator repository](https://github.com/rust-lang-ua/rust_incubator). Read more about how to do this in the [instructions](https://github.com/rust-lang-ua/rust_incubator#before-you-start).

#### - How and where to send Pull Requests of completed tasks?

In your private fork, you create a separate branch for each subdivision (task). In the corresponding branch, you complete the task and create a PR to __your__ main master branch. Don't forget to tag the reviewers in your PR.

<!-- #### - Що за табличка прогресу та як нею користуватись?
Усім учасникам на електронну адресу прийшло посилання на гугл-табличку. У ній є головна сторінка "Main Page", там відповідно до групи знаходите себе, а також номер, під яким ви записані. Тепер внизу шукаєте аркуш, який є під вашим номером. Це ваша сторінка, яку ви можете редагувати. Додаєте сюди фото. Навпроти кожного підрозділу стоять чекбокси, які вам потрібно відмічати при виконанні завдань. Також у відповідній колонці залишати посилання на виконане завдання. На особистій сторінці можете відслідковувати власний прогрес по розділах, а у 'Main Page'  загальний ваш та колег. -->

#### - When is a task accepted as completed?

When you've made a Pull Request and it's been approved and stale by one of the reviewers.

#### - What to do if the task is unfinished? Should I open a PR?

Yes, you should open a PR as soon as possible and indicate "Not Ready" in the title.

#### - Can I do the course assignments only on weekends?

You should devote more than 20 hours a week to your studies to achieve quality results. In 3 months of the intensive course, you can grow as much as in two years of independent practice. If you leave the task only on weekends, you will study for 10 hours a day. It is better to rationally distribute your study time during the week.

#### - Where to answer the questions in the first chapter?

Leave the answers under the question itself by editing your own md-file.

#### - Where can I ask for help?

You can ask questions in the Telegram [chat of our community](https://t.me/rustlang_ua), the official [forum](https://users.rust-lang.org/), [ChatGPT](https://openai.com/blog/chatgpt) for simple questions, and the bootcamp chat, including other participants. However your solution must be original. Both the person who gave the solution and the person who copied it will be expelled.

<!-- xxx : extend, tell about forum and chat of community -->

#### - What is the best way to ask questions ?

It is better if you have the code in the [playground](https://play.rust-lang.org/). In this case, we don’t spend extra time recreating the problem. 
![playground_ask](https://github.com/rust-lang-ua/rust_incubator/assets/98274821/2351bddd-455f-4078-a7cb-328a7bb08ac9)

<!-- xxx : write please, add screenshot -->

#### - How do I remove changes from the main repository?

Add the main repository as a remote.

```bash
# додайте remote
git remote add template git@github.com:rust-lang-ua/rust_incubator.git
# Заберіть зміни з репозиторію
git fetch --all
# Змерджіть зміни
git merge template/master
```

*If you get the error `fatal: refusing to merge unrelated histories` додайте `--allow-unrelated-histories`  flag to the last command.*
