document.addEventListener("DOMContentLoaded", _ => {
  const userChangerButton = document.getElementById('user-changer');
  userChangerButton.addEventListener('click', _ => {
    changeUser();
  });
});

function changeUser() {
  localStorage.removeItem("userId");
  window.location.href = "/";
}
