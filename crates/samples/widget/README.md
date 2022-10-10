# Windows widget sample

## Overview
Windows widgets are small UI containers that display text and graphics, associated with an app installed on the device.
For more information, see https://learn.microsoft.com/windows/apps/design/widgets/.

This sample widget crate contains several crates within:
  * `windows_widgets`: Windows widget API bindings
  * `gen`: Tooling to generate the windows_widgets crate
  * `provider`: Windows widget provider

## Prerequisites
* Windows Insider Dev Channel build 25211 or later
* Widgets board version 521.20060.1205.0 or later
* Windows App SDK 1.2 Preview 2 runtime (not needed if published to the Microsoft Store)
  https://learn.microsoft.com/windows/apps/windows-app-sdk/downloads#windows-app-sdk-12

## Running the sample
1. `cargo build`
2. `./deploy.ps1`

## Debugging tips

### Make the provider easier to debug/terminate

The sample widget provider is currently implemented as an out-of-process executable server compiled for the
`windows` subsystem and has no UI. Removing (or changing) the `#![windows_subsystem = "windows"]` attribute
will retarget the provider to run in the `console` subsystem, resulting in a console window that can be used
for diagnostic output and easier lifetime control.
