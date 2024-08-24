# Hotel Reservation API Abstraction Challenge

## Description
The [initial code](https://github.com/welf/reservation-service-challenge/blob/initial-code/src/main.rs) is located [in the `initial-code` branch](https://github.com/welf/reservation-service-challenge/tree/initial-code) and is a simple implementation of the API for a hotel reservation system using [Axum](https://docs.rs/axum/latest/axum/) server. Using the API, it is possible to get reservations, create a new reservation for new and existing customers, and cancel a reservation by deleting it. After the each action the server sends an email to the client confirming the action was successfuly performed. The API is tightle coupled with the simple in-memory database (HashMap) to store the reservations, to the implementation of the mailer service, and can send emails only.

The goal of the challenge is to refactor the code to make it more modular, testable, and maintainable. The code should be refactored to use the repository pattern to abstract the data storage, and the mailer service should be abstracted to allow for different implementations and be able to send not only emails but also other types of messages.

Each API endpoint handler has a corresponding test and the tests should still pass after the refactoring.
