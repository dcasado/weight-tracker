document.addEventListener("DOMContentLoaded", _ => {
  const startDateInput = document.getElementById('start-date');
  startDateInput.addEventListener("change", function () {
    const endDate = document.getElementById('end-date').value;
    if (this.value && endDate) {
      window.location.href = window.location.pathname + "?start-date=" + this.value + "&end-date=" + endDate;
    }
  });

  const endDateInput = document.getElementById('end-date');
  endDateInput.max = new Date().toISOString().split("T")[0];
  endDateInput.addEventListener("change", function () {
    const startDate = document.getElementById('start-date').value;
    if (this.value && startDate) {
      window.location.href = window.location.pathname + "?start-date=" + startDate + "&end-date=" + this.value;
    }
  });

  const chartDiv = document.getElementById("chart-div");
  const dates = JSON.parse(chartDiv.getAttribute("data-js-dates"));
  const weights = JSON.parse(chartDiv.getAttribute("data-js-weights"));

  const weightChart = document.getElementById('weight-chart');
  chart = new Chart(weightChart, {
    type: 'line',
    options: {
      responsive: true,
      maintainAspectRatio: false,
      animation: false
    },
    data: {
      labels: dates,
      datasets: [{
        label: 'Weight',
        data: weights,
        borderWidth: 1,
        tension: 0.25,
        spanGaps: true
      }]
    }
  });
});
