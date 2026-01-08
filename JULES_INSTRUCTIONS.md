# Jules Instructions: Local Community Feed

## Project Status

This project is a local-first social feed application built with Tauri, React, and Rust. The backend is mostly complete, with functioning modules for cryptography, database, and networking. The frontend has been scaffolded with basic components and styling.

## Blocking Issues

The primary blocker is an environmental issue that prevents the Tauri application from launching. The application fails to start with the error `Xvfb failed to start`, which indicates a problem with the graphical environment. I have been unable to resolve this issue, even after installing `Xvfb` and using `xvfb-run`.

## Next Steps

1.  **Resolve the environmental issue:** The next Jules agent will need to find a solution to the `Xvfb failed to start` error. This may involve further investigation into the environment's configuration or trying a different approach to running the application in a headless environment.
2.  **Complete the frontend implementation:** Once the application can be launched, the frontend needs to be connected to the backend. This will involve using the `@tauri-apps/api` library to call the backend commands and display the data in the UI.
3.  **Perform end-to-end testing:** Once the frontend and backend are connected, a full end-to-end test should be performed to ensure that all features are working correctly.
4.  **Complete pre-commit steps and submit:** Once the application is fully functional and tested, the pre-commit steps should be completed, and the code should be submitted.
