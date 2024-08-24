# Hotel Reservation API Abstraction Challenge

## Description
The code is a simple implementation of the API for a hotel reservation system using Axum server. Using the API, it to get reservations, create a new reservation for new and existing customers, and cancel a reservation. The API is tightle coupled with the simple in-memory database (HashMap) to store the reservations, to the implementation of the mailer service, and can send emails only.

The goal of the challenge is to refactor the code to make it more modular, testable, and maintainable. The code should be refactored to use the repository pattern to abstract the data storage, and the mailer service should be abstracted to allow for different implementations and be able to send not only emails but also other types of messages.

Each API endpoint handler has a corresponding test and the tests should still pass after the refactoring.
