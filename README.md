# App Test

## Requirements 
An application that will work as a service that will be running in the background.
The application can be "started" in 2 main ways:
1. by opening it in the system tray menu
2. by opening with some kind of input
    * http requests
    * file input

When the application is started by opening it in the system tray menu, some options will be available, the user selects them e do stuff, then closes it.

About the other methods:
<br>
**File watcher**: the app will be monitoring a directory, when a file with a defined name and extension is created in that directory, it will read it, parse it, and start the application, do stuff, and then close it. 
<br>
**Server**: the app will be listening in a http server, when a request happends, it will  start the application, do stuff, and then close it. 

<br>

## Reasoning
My reasoning using Tauri to create this application. 

<br>
The tauri application will start in the main thread, in the setup of tauri application, we start a thread called maestro, that will orquestrate the other threads.

<br>
The maestro thread will start the monitoring threads (server && file watcher). As soon as one of the monitoring threads receive an input, the maestro thread will tell the monitoring threads to stop monitoring, it will receive the input values, open the tauri window, do stuff, and when it closes, it will send back to the monitoring thread the output, that the monitoring thread will expose (file watcher creates an output file, server responds the output in the /status endpoint);

## Architecture

![image](https://github.com/gabref/tauri-tests/assets/83483325/5b600c05-ae44-4647-b260-5b90131bf08b)

## Annoing bug still to solve
When you start the application with `cargo tauri dev`, if in the developer console you see an error, just close and restart until the error does not show up. It is an error with nextjs 14. I tried to downgrade it to nextjs 13, but then tailwind does not work, still have to get that right.

## Test The App

**File watcher** not implemented yet.

**Server**
You can start the application with:
curl localhost:8080/start   -> will start a window that shows random info, and then closes
curl localhost:8080/wai     -> start a window, waits for user interaction, user has to close the window
curl localhost:8080/status  -> shows the status of the last transaction

## TODOs

menu -> system tray and on app opened

splashscreen -> change at runtime the image / .html
ENV_VARIABLES?
images in embed server -> download at ativation and then use that for next startups

screens rendered by route or by events from the rust side?

whitelabel icon -> embed
whitelabel icon -> 1. pipeline specific user 2. at runtime change icon
