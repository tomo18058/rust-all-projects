let currentIndex = 0;
let score = 0;
let quizData = [];

window.onload = async function () {
    const res = await fetch('/api/quizzes');
    quizData = await res.json();
    showQuestion();
};

function showQuestion() {
    const quiz = quizData[currentIndex];
    document.getElementById("code-block").innerHTML = quiz.question.replace(/\\n/g,"<br>")                                                                                     
    document.getElementById("result").textContent = "";
    document.getElementById("next-button").style.display = "none";
    updateStatus();
}

function checkAnswer(selected) {
    const quiz = quizData[currentIndex];
    const result = document.getElementById("result")
    if (selected === quiz.answer) {
        result.textContent = "正解！";
        result.style.color = "lightblue";
        score++;
    } else {
        result.textContent = "残念、不正解！";
        result.style.color = "red";
    }

    document.getElementById("next-button").style.display = "inline-block";
    updateStatus();
}

function loadNextQuestion() {
    currentIndex++;
    if (currentIndex < quizData.length) {
        showQuestion();
    } else {
        document.getElementById("code-block").textContent = "クイズ終了！お疲れさまでした！";
        document.querySelector(".options").style.display = "none";
        document.getElementById("next-button").style.display = "none";
    }
}

function updateStatus() {
    document.getElementById("question-count").textContent = `${currentIndex + 1}問目`;
    document.getElementById("score").textContent = `正解数: ${score}`;
}

function setTheme(Theme) {
    document.body.className = theme + '-theme';
}

function setTheme(theme) {
    document.body.classList.remove("rust-theme", "js-theme");
    if (theme === "rust") {
        document.body.classList.add("rust-theme");
    } else if (theme === "js") {
        document.body.classList.add("js-theme");
    }
}