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

  document.querySelectorAll("[data-js-delete-measurement]").forEach(element => {
    element.addEventListener('click', (event) => {
      let id = element.getAttribute('data-js-delete-measurement');
      deleteMeasurement(id);
    });
  });
});

function deleteMeasurement(id) {
  const confirmText = "Do you really want to delete the measurement?";
  if (confirm(confirmText) === true) {
    fetch('/api/measurements/' + id, {
      method: 'DELETE',
    }).then(response => {
      window.location.reload();
    });
  }
}
