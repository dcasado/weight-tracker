# Weight Tracker

The name of this project is pretty self explanatory. This is a service to track the weight of multiple people and display it on a chart.

# Features

- Register weight to a specific user
- Retrieve weight for a particular user filtering by date range
- Delete a weight entry

# Technologies used

The backend is implemented on rust using the axum framework while the front end is plain html, css and javascript with the library Charts.js for the visualizations and handlebars for the templates. It uses sqlite as database.

# Development

To be able to compile the project for the first time you have to first configure a database with a valid schema. Because there is already a `.env` file with the database URL the commands below will work without arguments.
 - Install sqlx-cli `cargo install sqlx-cli`.
 - Create the database `sqlx database create`.
 - Apply the migrations `sqlx migrate run`.
 - You may need to reload your IDE if it was opened before the database file was created.

After that you should be able to compile the project with `cargo build`.
