let userId = localStorage.getItem("userId");
if (userId !== null) {
  window.location.replace("/chart/" + userId)
}

document.addEventListener("DOMContentLoaded", _ => {
  document.querySelectorAll("[data-js-select-user]").forEach(element => {
    element.addEventListener("click", _ => {
      const id = element.getAttribute("data-js-select-user");
      goToChart(id);
    });
  });
});

function goToChart(id) {
  localStorage.setItem("userId", id);
  window.location.href = "/chart/" + id;
}
