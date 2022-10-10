xcopy /Y /E "crates\provider\packaging\*" target\debug\
Get-AppxPackage -Publisher CN=WidgetProviderIdentity | Remove-AppxPackage
Add-AppxPackage -Register "target\debug\appxmanifest.xml"
