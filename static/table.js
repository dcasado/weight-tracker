document.addEventListener("DOMContentLoaded", _ => {
  let yearSelect = document.getElementById('year-select');
  yearSelect.value = yearSelect.getAttribute('data-js-selected');

  let monthSelect = document.getElementById('month-select');
  monthSelect.value = monthSelect.getAttribute('data-js-selected');

  yearSelect.addEventListener("change", function () {
    window.location.href = window.location.pathname + "?year=" + this.value;
  });

  monthSelect.addEventListener("change", function () {
    let year = yearSelect.getAttribute('data-js-selected')
    window.location.href = window.location.pathname + "?year=" + year + "&month=" + this.value;
  });

  document.querySelectorAll("[data-js-delete-weight]").forEach(element => {
    element.addEventListener('click', (event) => {
      let weight_id = element.getAttribute('data-js-delete-weight');
      deleteWeight(weight_id);
    });
  });
});

function deleteWeight(weight_id) {
  const confirmText = "Do you really want to delete the weight?";
  if (confirm(confirmText) === true) {
    fetch('/api/measurements/weights/' + weight_id, {
      method: 'DELETE',
    }).then(response => {
      window.location.reload();
    });
  }
}
