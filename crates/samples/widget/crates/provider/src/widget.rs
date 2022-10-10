use std::sync::atomic::{AtomicU32, Ordering};

use windows::core::*;
use windows_widgets::Providers::{IWidgetProvider, IWidgetProvider_Impl, WidgetActionInvokedArgs, WidgetContext, WidgetContextChangedArgs, WidgetManager, WidgetUpdateRequestOptions};

#[implement(IWidgetProvider)]
pub struct WidgetProvider {
    click_count: AtomicU32,
}

impl Default for WidgetProvider {
    fn default() -> Self {
        Self { click_count: AtomicU32::new(0) }
    }
}

#[allow(non_snake_case)]
impl WidgetProvider {
    fn update_widget_data(&self, id: &HSTRING) -> Result<()> {
        let options = WidgetUpdateRequestOptions::CreateInstance(id)?;
        options.SetData(&HSTRING::from(format!(r##"{{ "count": "{}" }}"##, &self.click_count.load(Ordering::Relaxed)))).unwrap();
        WidgetManager::GetDefault()?.UpdateWidget(&options)
    }

    fn update_widget_template(&self, id: &HSTRING) -> Result<()> {
        let options = WidgetUpdateRequestOptions::CreateInstance(id)?;
        options.SetTemplate(w!(r##"{
            "type": "AdaptiveCard",
            "body": [
                {
                    "type": "Container",
                    "items": [
                        {
                            "type": "ColumnSet",
                            "columns": [
                                {
                                    "type": "Column",
                                    "width": "stretch",
                                    "items": [
                                        {
                                            "type": "TextBlock",
                                            "text": "Click count: ${count}",
                                            "wrap": true,
                                            "size": "Large",
                                            "weight": "Default"
                                        }
                                    ]
                                },
                                {
                                    "type": "Column",
                                    "width": "auto",
                                    "items": [
                                        {
                                            "type": "Image",
                                            "url": "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjxzdmcKICAgdmVyc2lvbj0iMS4xIgogICBoZWlnaHQ9IjEwNiIKICAgd2lkdGg9IjEwNiIKICAgaWQ9InN2ZzQwNSIKICAgc29kaXBvZGk6ZG9jbmFtZT0iUnVzdF9wcm9ncmFtbWluZ19sYW5ndWFnZV9ibGFja19sb2dvLnN2ZyIKICAgaW5rc2NhcGU6dmVyc2lvbj0iMS4yLjEgKDljNmQ0MWU0MTAsIDIwMjItMDctMTQpIgogICB4bWxuczppbmtzY2FwZT0iaHR0cDovL3d3dy5pbmtzY2FwZS5vcmcvbmFtZXNwYWNlcy9pbmtzY2FwZSIKICAgeG1sbnM6c29kaXBvZGk9Imh0dHA6Ly9zb2RpcG9kaS5zb3VyY2Vmb3JnZS5uZXQvRFREL3NvZGlwb2RpLTAuZHRkIgogICB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIKICAgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIgogICB4bWxuczpzdmc9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8ZGVmcwogICAgIGlkPSJkZWZzNDA5IiAvPgogIDxzb2RpcG9kaTpuYW1lZHZpZXcKICAgICBpZD0ibmFtZWR2aWV3NDA3IgogICAgIHBhZ2Vjb2xvcj0iI2ZmZmZmZiIKICAgICBib3JkZXJjb2xvcj0iIzAwMDAwMCIKICAgICBib3JkZXJvcGFjaXR5PSIwLjI1IgogICAgIGlua3NjYXBlOnNob3dwYWdlc2hhZG93PSIyIgogICAgIGlua3NjYXBlOnBhZ2VvcGFjaXR5PSIwLjAiCiAgICAgaW5rc2NhcGU6cGFnZWNoZWNrZXJib2FyZD0iMCIKICAgICBpbmtzY2FwZTpkZXNrY29sb3I9IiNkMWQxZDEiCiAgICAgc2hvd2dyaWQ9ImZhbHNlIgogICAgIGlua3NjYXBlOnpvb209IjIuODI4NDI3MSIKICAgICBpbmtzY2FwZTpjeD0iNTUuNjg0NjU5IgogICAgIGlua3NjYXBlOmN5PSI1Ni4wMzgyMTIiCiAgICAgaW5rc2NhcGU6d2luZG93LXdpZHRoPSI4ODIiCiAgICAgaW5rc2NhcGU6d2luZG93LWhlaWdodD0iNjYwIgogICAgIGlua3NjYXBlOndpbmRvdy14PSIyMTEyIgogICAgIGlua3NjYXBlOndpbmRvdy15PSI5OSIKICAgICBpbmtzY2FwZTp3aW5kb3ctbWF4aW1pemVkPSIwIgogICAgIGlua3NjYXBlOmN1cnJlbnQtbGF5ZXI9ImdlYXIiIC8+CiAgPGcKICAgICBpZD0ibG9nbyIKICAgICB0cmFuc2Zvcm09InRyYW5zbGF0ZSg1MywgNTMpIj4KICAgIDxwYXRoCiAgICAgICBpZD0iciIKICAgICAgIHN0cm9rZT0iYmxhY2siCiAgICAgICBzdHJva2Utd2lkdGg9IjEiCiAgICAgICBzdHJva2UtbGluZWpvaW49InJvdW5kIgogICAgICAgZD0iICAgICBNIC05LC0xNSBIIDQgQyAxMiwtMTUgMTIsLTcgNCwtNyBIIC05IFogICAgIE0gLTQwLDIyIEggMCBWIDExIEggLTkgViAzIEggMSBDIDEyLDMgNiwyMiAxNSwyMiBIIDQwICAgICBWIDMgSCAzNCBWIDUgQyAzNCwxMyAyNSwxMiAyNCw3IEMgMjMsMiAxOSwtMiAxOCwtMiBDIDMzLC0xMCAyNCwtMjYgMTIsLTI2IEggLTM1ICAgICBWIC0xNSBIIC0yNSBWIDExIEggLTQwIFoiCiAgICAgICB0cmFuc2Zvcm09InRyYW5zbGF0ZSgwLjUsIDAuNSkiCiAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MTtmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjEiIC8+CiAgICA8ZwogICAgICAgaWQ9ImdlYXIiCiAgICAgICBtYXNrPSJ1cmwoI2hvbGVzKSIKICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxIj4KICAgICAgPGNpcmNsZQogICAgICAgICByPSI0MyIKICAgICAgICAgZmlsbD0ibm9uZSIKICAgICAgICAgc3Ryb2tlPSJibGFjayIKICAgICAgICAgc3Ryb2tlLXdpZHRoPSI5IgogICAgICAgICBpZD0iY2lyY2xlMzE1IgogICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MSIgLz4KICAgICAgPGcKICAgICAgICAgaWQ9ImNvZ3MiCiAgICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxIj4KICAgICAgICA8cG9seWdvbgogICAgICAgICAgIGlkPSJjb2ciCiAgICAgICAgICAgc3Ryb2tlPSJibGFjayIKICAgICAgICAgICBzdHJva2Utd2lkdGg9IjMiCiAgICAgICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgICAgICBwb2ludHM9IjQ2LDMgNTEsMCA0NiwtMyIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MSIgLz4KICAgICAgICA8dXNlCiAgICAgICAgICAgeGxpbms6aHJlZj0iI2NvZyIKICAgICAgICAgICB0cmFuc2Zvcm09InJvdGF0ZSgxMS4yNSkiCiAgICAgICAgICAgaWQ9InVzZTMxOCIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MTtmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjEiIC8+CiAgICAgICAgPHVzZQogICAgICAgICAgIHhsaW5rOmhyZWY9IiNjb2ciCiAgICAgICAgICAgdHJhbnNmb3JtPSJyb3RhdGUoMjIuNTApIgogICAgICAgICAgIGlkPSJ1c2UzMjAiCiAgICAgICAgICAgc3R5bGU9InN0cm9rZTojZmZmZmZmO3N0cm9rZS1vcGFjaXR5OjE7ZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxIiAvPgogICAgICAgIDx1c2UKICAgICAgICAgICB4bGluazpocmVmPSIjY29nIgogICAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDMzLjc1KSIKICAgICAgICAgICBpZD0idXNlMzIyIgogICAgICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxO2ZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MSIgLz4KICAgICAgICA8dXNlCiAgICAgICAgICAgeGxpbms6aHJlZj0iI2NvZyIKICAgICAgICAgICB0cmFuc2Zvcm09InJvdGF0ZSg0NS4wMCkiCiAgICAgICAgICAgaWQ9InVzZTMyNCIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MTtmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjEiIC8+CiAgICAgICAgPHVzZQogICAgICAgICAgIHhsaW5rOmhyZWY9IiNjb2ciCiAgICAgICAgICAgdHJhbnNmb3JtPSJyb3RhdGUoNTYuMjUpIgogICAgICAgICAgIGlkPSJ1c2UzMjYiCiAgICAgICAgICAgc3R5bGU9InN0cm9rZTojZmZmZmZmO3N0cm9rZS1vcGFjaXR5OjE7ZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxIiAvPgogICAgICAgIDx1c2UKICAgICAgICAgICB4bGluazpocmVmPSIjY29nIgogICAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDY3LjUwKSIKICAgICAgICAgICBpZD0idXNlMzI4IgogICAgICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxO2ZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MSIgLz4KICAgICAgICA8dXNlCiAgICAgICAgICAgeGxpbms6aHJlZj0iI2NvZyIKICAgICAgICAgICB0cmFuc2Zvcm09InJvdGF0ZSg3OC43NSkiCiAgICAgICAgICAgaWQ9InVzZTMzMCIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MTtmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjEiIC8+CiAgICAgICAgPHVzZQogICAgICAgICAgIHhsaW5rOmhyZWY9IiNjb2ciCiAgICAgICAgICAgdHJhbnNmb3JtPSJyb3RhdGUoOTAuMDApIgogICAgICAgICAgIGlkPSJ1c2UzMzIiCiAgICAgICAgICAgc3R5bGU9InN0cm9rZTojZmZmZmZmO3N0cm9rZS1vcGFjaXR5OjE7ZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxIiAvPgogICAgICAgIDx1c2UKICAgICAgICAgICB4bGluazpocmVmPSIjY29nIgogICAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDEwMS4yNSkiCiAgICAgICAgICAgaWQ9InVzZTMzNCIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MTtmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjEiIC8+CiAgICAgICAgPHVzZQogICAgICAgICAgIHhsaW5rOmhyZWY9IiNjb2ciCiAgICAgICAgICAgdHJhbnNmb3JtPSJyb3RhdGUoMTEyLjUwKSIKICAgICAgICAgICBpZD0idXNlMzM2IgogICAgICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxO2ZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MSIgLz4KICAgICAgICA8dXNlCiAgICAgICAgICAgeGxpbms6aHJlZj0iI2NvZyIKICAgICAgICAgICB0cmFuc2Zvcm09InJvdGF0ZSgxMjMuNzUpIgogICAgICAgICAgIGlkPSJ1c2UzMzgiCiAgICAgICAgICAgc3R5bGU9InN0cm9rZTojZmZmZmZmO3N0cm9rZS1vcGFjaXR5OjE7ZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxIiAvPgogICAgICAgIDx1c2UKICAgICAgICAgICB4bGluazpocmVmPSIjY29nIgogICAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDEzNS4wMCkiCiAgICAgICAgICAgaWQ9InVzZTM0MCIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MTtmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjEiIC8+CiAgICAgICAgPHVzZQogICAgICAgICAgIHhsaW5rOmhyZWY9IiNjb2ciCiAgICAgICAgICAgdHJhbnNmb3JtPSJyb3RhdGUoMTQ2LjI1KSIKICAgICAgICAgICBpZD0idXNlMzQyIgogICAgICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxO2ZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MSIgLz4KICAgICAgICA8dXNlCiAgICAgICAgICAgeGxpbms6aHJlZj0iI2NvZyIKICAgICAgICAgICB0cmFuc2Zvcm09InJvdGF0ZSgxNTcuNTApIgogICAgICAgICAgIGlkPSJ1c2UzNDQiCiAgICAgICAgICAgc3R5bGU9InN0cm9rZTojZmZmZmZmO3N0cm9rZS1vcGFjaXR5OjE7ZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxIiAvPgogICAgICAgIDx1c2UKICAgICAgICAgICB4bGluazpocmVmPSIjY29nIgogICAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDE2OC43NSkiCiAgICAgICAgICAgaWQ9InVzZTM0NiIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MTtmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjEiIC8+CiAgICAgICAgPHVzZQogICAgICAgICAgIHhsaW5rOmhyZWY9IiNjb2ciCiAgICAgICAgICAgdHJhbnNmb3JtPSJyb3RhdGUoMTgwLjAwKSIKICAgICAgICAgICBpZD0idXNlMzQ4IgogICAgICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxO2ZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MSIgLz4KICAgICAgICA8dXNlCiAgICAgICAgICAgeGxpbms6aHJlZj0iI2NvZyIKICAgICAgICAgICB0cmFuc2Zvcm09InJvdGF0ZSgxOTEuMjUpIgogICAgICAgICAgIGlkPSJ1c2UzNTAiCiAgICAgICAgICAgc3R5bGU9InN0cm9rZTojZmZmZmZmO3N0cm9rZS1vcGFjaXR5OjE7ZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxIiAvPgogICAgICAgIDx1c2UKICAgICAgICAgICB4bGluazpocmVmPSIjY29nIgogICAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDIwMi41MCkiCiAgICAgICAgICAgaWQ9InVzZTM1MiIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MTtmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjEiIC8+CiAgICAgICAgPHVzZQogICAgICAgICAgIHhsaW5rOmhyZWY9IiNjb2ciCiAgICAgICAgICAgdHJhbnNmb3JtPSJyb3RhdGUoMjEzLjc1KSIKICAgICAgICAgICBpZD0idXNlMzU0IgogICAgICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxO2ZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MSIgLz4KICAgICAgICA8dXNlCiAgICAgICAgICAgeGxpbms6aHJlZj0iI2NvZyIKICAgICAgICAgICB0cmFuc2Zvcm09InJvdGF0ZSgyMjUuMDApIgogICAgICAgICAgIGlkPSJ1c2UzNTYiCiAgICAgICAgICAgc3R5bGU9InN0cm9rZTojZmZmZmZmO3N0cm9rZS1vcGFjaXR5OjE7ZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxIiAvPgogICAgICAgIDx1c2UKICAgICAgICAgICB4bGluazpocmVmPSIjY29nIgogICAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDIzNi4yNSkiCiAgICAgICAgICAgaWQ9InVzZTM1OCIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MTtmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjEiIC8+CiAgICAgICAgPHVzZQogICAgICAgICAgIHhsaW5rOmhyZWY9IiNjb2ciCiAgICAgICAgICAgdHJhbnNmb3JtPSJyb3RhdGUoMjQ3LjUwKSIKICAgICAgICAgICBpZD0idXNlMzYwIgogICAgICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxO2ZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MSIgLz4KICAgICAgICA8dXNlCiAgICAgICAgICAgeGxpbms6aHJlZj0iI2NvZyIKICAgICAgICAgICB0cmFuc2Zvcm09InJvdGF0ZSgyNTguNzUpIgogICAgICAgICAgIGlkPSJ1c2UzNjIiCiAgICAgICAgICAgc3R5bGU9InN0cm9rZTojZmZmZmZmO3N0cm9rZS1vcGFjaXR5OjE7ZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxIiAvPgogICAgICAgIDx1c2UKICAgICAgICAgICB4bGluazpocmVmPSIjY29nIgogICAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDI3MC4wMCkiCiAgICAgICAgICAgaWQ9InVzZTM2NCIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MTtmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjEiIC8+CiAgICAgICAgPHVzZQogICAgICAgICAgIHhsaW5rOmhyZWY9IiNjb2ciCiAgICAgICAgICAgdHJhbnNmb3JtPSJyb3RhdGUoMjgxLjI1KSIKICAgICAgICAgICBpZD0idXNlMzY2IgogICAgICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxO2ZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MSIgLz4KICAgICAgICA8dXNlCiAgICAgICAgICAgeGxpbms6aHJlZj0iI2NvZyIKICAgICAgICAgICB0cmFuc2Zvcm09InJvdGF0ZSgyOTIuNTApIgogICAgICAgICAgIGlkPSJ1c2UzNjgiCiAgICAgICAgICAgc3R5bGU9InN0cm9rZTojZmZmZmZmO3N0cm9rZS1vcGFjaXR5OjE7ZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxIiAvPgogICAgICAgIDx1c2UKICAgICAgICAgICB4bGluazpocmVmPSIjY29nIgogICAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDMwMy43NSkiCiAgICAgICAgICAgaWQ9InVzZTM3MCIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MTtmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjEiIC8+CiAgICAgICAgPHVzZQogICAgICAgICAgIHhsaW5rOmhyZWY9IiNjb2ciCiAgICAgICAgICAgdHJhbnNmb3JtPSJyb3RhdGUoMzE1LjAwKSIKICAgICAgICAgICBpZD0idXNlMzcyIgogICAgICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxO2ZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MSIgLz4KICAgICAgICA8dXNlCiAgICAgICAgICAgeGxpbms6aHJlZj0iI2NvZyIKICAgICAgICAgICB0cmFuc2Zvcm09InJvdGF0ZSgzMjYuMjUpIgogICAgICAgICAgIGlkPSJ1c2UzNzQiCiAgICAgICAgICAgc3R5bGU9InN0cm9rZTojZmZmZmZmO3N0cm9rZS1vcGFjaXR5OjE7ZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxIiAvPgogICAgICAgIDx1c2UKICAgICAgICAgICB4bGluazpocmVmPSIjY29nIgogICAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDMzNy41MCkiCiAgICAgICAgICAgaWQ9InVzZTM3NiIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MTtmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjEiIC8+CiAgICAgICAgPHVzZQogICAgICAgICAgIHhsaW5rOmhyZWY9IiNjb2ciCiAgICAgICAgICAgdHJhbnNmb3JtPSJyb3RhdGUoMzQ4Ljc1KSIKICAgICAgICAgICBpZD0idXNlMzc4IgogICAgICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxO2ZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MSIgLz4KICAgICAgPC9nPgogICAgICA8ZwogICAgICAgICBpZD0ibW91bnRzIgogICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MSI+CiAgICAgICAgPHBvbHlnb24KICAgICAgICAgICBpZD0ibW91bnQiCiAgICAgICAgICAgc3Ryb2tlPSJibGFjayIKICAgICAgICAgICBzdHJva2Utd2lkdGg9IjYiCiAgICAgICAgICAgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIKICAgICAgICAgICBwb2ludHM9Ii03LC00MiAwLC0zNSA3LC00MiIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MSIgLz4KICAgICAgICA8dXNlCiAgICAgICAgICAgeGxpbms6aHJlZj0iI21vdW50IgogICAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDcyKSIKICAgICAgICAgICBpZD0idXNlMzgyIgogICAgICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxO2ZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MSIgLz4KICAgICAgICA8dXNlCiAgICAgICAgICAgeGxpbms6aHJlZj0iI21vdW50IgogICAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDE0NCkiCiAgICAgICAgICAgaWQ9InVzZTM4NCIKICAgICAgICAgICBzdHlsZT0ic3Ryb2tlOiNmZmZmZmY7c3Ryb2tlLW9wYWNpdHk6MTtmaWxsOiNmZmZmZmY7ZmlsbC1vcGFjaXR5OjEiIC8+CiAgICAgICAgPHVzZQogICAgICAgICAgIHhsaW5rOmhyZWY9IiNtb3VudCIKICAgICAgICAgICB0cmFuc2Zvcm09InJvdGF0ZSgyMTYpIgogICAgICAgICAgIGlkPSJ1c2UzODYiCiAgICAgICAgICAgc3R5bGU9InN0cm9rZTojZmZmZmZmO3N0cm9rZS1vcGFjaXR5OjE7ZmlsbDojZmZmZmZmO2ZpbGwtb3BhY2l0eToxIiAvPgogICAgICAgIDx1c2UKICAgICAgICAgICB4bGluazpocmVmPSIjbW91bnQiCiAgICAgICAgICAgdHJhbnNmb3JtPSJyb3RhdGUoMjg4KSIKICAgICAgICAgICBpZD0idXNlMzg4IgogICAgICAgICAgIHN0eWxlPSJzdHJva2U6I2ZmZmZmZjtzdHJva2Utb3BhY2l0eToxO2ZpbGw6I2ZmZmZmZjtmaWxsLW9wYWNpdHk6MSIgLz4KICAgICAgPC9nPgogICAgPC9nPgogICAgPG1hc2sKICAgICAgIGlkPSJob2xlcyI+CiAgICAgIDxyZWN0CiAgICAgICAgIHg9Ii02MCIKICAgICAgICAgeT0iLTYwIgogICAgICAgICB3aWR0aD0iMTIwIgogICAgICAgICBoZWlnaHQ9IjEyMCIKICAgICAgICAgZmlsbD0id2hpdGUiCiAgICAgICAgIGlkPSJyZWN0MzkyIiAvPgogICAgICA8Y2lyY2xlCiAgICAgICAgIGlkPSJob2xlIgogICAgICAgICBjeT0iLTQwIgogICAgICAgICByPSIzIiAvPgogICAgICA8dXNlCiAgICAgICAgIHhsaW5rOmhyZWY9IiNob2xlIgogICAgICAgICB0cmFuc2Zvcm09InJvdGF0ZSg3MikiCiAgICAgICAgIGlkPSJ1c2UzOTUiIC8+CiAgICAgIDx1c2UKICAgICAgICAgeGxpbms6aHJlZj0iI2hvbGUiCiAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDE0NCkiCiAgICAgICAgIGlkPSJ1c2UzOTciIC8+CiAgICAgIDx1c2UKICAgICAgICAgeGxpbms6aHJlZj0iI2hvbGUiCiAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDIxNikiCiAgICAgICAgIGlkPSJ1c2UzOTkiIC8+CiAgICAgIDx1c2UKICAgICAgICAgeGxpbms6aHJlZj0iI2hvbGUiCiAgICAgICAgIHRyYW5zZm9ybT0icm90YXRlKDI4OCkiCiAgICAgICAgIGlkPSJ1c2U0MDEiIC8+CiAgICA8L21hc2s+CiAgPC9nPgo8L3N2Zz4K",
                                            "size": "Large",
                                            "height": "45px",
                                            "horizontalAlignment": "Left",
                                            "$when": "${$host.hostTheme==\"dark\"}"
                                        },
                                        {
                                            "type": "Image",
                                            "url": "data:image/svg+xml;base64,PHN2ZyB2ZXJzaW9uPSIxLjEiIGhlaWdodD0iMTA2IiB3aWR0aD0iMTA2IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIj4KPGcgaWQ9ImxvZ28iIHRyYW5zZm9ybT0idHJhbnNsYXRlKDUzLCA1MykiPgogIDxwYXRoIGlkPSJyIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSgwLjUsIDAuNSkiIHN0cm9rZT0iYmxhY2siIHN0cm9rZS13aWR0aD0iMSIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIgZD0iICAgICBNIC05LC0xNSBIIDQgQyAxMiwtMTUgMTIsLTcgNCwtNyBIIC05IFogICAgIE0gLTQwLDIyIEggMCBWIDExIEggLTkgViAzIEggMSBDIDEyLDMgNiwyMiAxNSwyMiBIIDQwICAgICBWIDMgSCAzNCBWIDUgQyAzNCwxMyAyNSwxMiAyNCw3IEMgMjMsMiAxOSwtMiAxOCwtMiBDIDMzLC0xMCAyNCwtMjYgMTIsLTI2IEggLTM1ICAgICBWIC0xNSBIIC0yNSBWIDExIEggLTQwIFoiLz4KICA8ZyBpZD0iZ2VhciIgbWFzaz0idXJsKCNob2xlcykiPgogICAgPGNpcmNsZSByPSI0MyIgZmlsbD0ibm9uZSIgc3Ryb2tlPSJibGFjayIgc3Ryb2tlLXdpZHRoPSI5Ii8+CiAgICA8ZyBpZD0iY29ncyI+CiAgICAgIDxwb2x5Z29uIGlkPSJjb2ciIHN0cm9rZT0iYmxhY2siIHN0cm9rZS13aWR0aD0iMyIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIgcG9pbnRzPSI0NiwzIDUxLDAgNDYsLTMiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSgxMS4yNSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSgyMi41MCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSgzMy43NSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSg0NS4wMCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSg1Ni4yNSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSg2Ny41MCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSg3OC43NSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSg5MC4wMCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSgxMDEuMjUpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2NvZyIgdHJhbnNmb3JtPSJyb3RhdGUoMTEyLjUwKSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNjb2ciIHRyYW5zZm9ybT0icm90YXRlKDEyMy43NSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSgxMzUuMDApIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2NvZyIgdHJhbnNmb3JtPSJyb3RhdGUoMTQ2LjI1KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNjb2ciIHRyYW5zZm9ybT0icm90YXRlKDE1Ny41MCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSgxNjguNzUpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2NvZyIgdHJhbnNmb3JtPSJyb3RhdGUoMTgwLjAwKSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNjb2ciIHRyYW5zZm9ybT0icm90YXRlKDE5MS4yNSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSgyMDIuNTApIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2NvZyIgdHJhbnNmb3JtPSJyb3RhdGUoMjEzLjc1KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNjb2ciIHRyYW5zZm9ybT0icm90YXRlKDIyNS4wMCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSgyMzYuMjUpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2NvZyIgdHJhbnNmb3JtPSJyb3RhdGUoMjQ3LjUwKSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNjb2ciIHRyYW5zZm9ybT0icm90YXRlKDI1OC43NSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSgyNzAuMDApIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2NvZyIgdHJhbnNmb3JtPSJyb3RhdGUoMjgxLjI1KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNjb2ciIHRyYW5zZm9ybT0icm90YXRlKDI5Mi41MCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSgzMDMuNzUpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2NvZyIgdHJhbnNmb3JtPSJyb3RhdGUoMzE1LjAwKSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNjb2ciIHRyYW5zZm9ybT0icm90YXRlKDMyNi4yNSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjY29nIiB0cmFuc2Zvcm09InJvdGF0ZSgzMzcuNTApIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2NvZyIgdHJhbnNmb3JtPSJyb3RhdGUoMzQ4Ljc1KSIvPgogICAgPC9nPgogICAgPGcgaWQ9Im1vdW50cyI+CiAgICAgIDxwb2x5Z29uIGlkPSJtb3VudCIgc3Ryb2tlPSJibGFjayIgc3Ryb2tlLXdpZHRoPSI2IiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBwb2ludHM9Ii03LC00MiAwLC0zNSA3LC00MiIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNtb3VudCIgdHJhbnNmb3JtPSJyb3RhdGUoNzIpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI21vdW50IiB0cmFuc2Zvcm09InJvdGF0ZSgxNDQpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI21vdW50IiB0cmFuc2Zvcm09InJvdGF0ZSgyMTYpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI21vdW50IiB0cmFuc2Zvcm09InJvdGF0ZSgyODgpIi8+CiAgICA8L2c+CiAgPC9nPgogIDxtYXNrIGlkPSJob2xlcyI+CiAgICA8cmVjdCB4PSItNjAiIHk9Ii02MCIgd2lkdGg9IjEyMCIgaGVpZ2h0PSIxMjAiIGZpbGw9IndoaXRlIi8+CiAgICA8Y2lyY2xlIGlkPSJob2xlIiBjeT0iLTQwIiByPSIzIi8+CiAgICA8dXNlIHhsaW5rOmhyZWY9IiNob2xlIiB0cmFuc2Zvcm09InJvdGF0ZSg3MikiLz4KICAgIDx1c2UgeGxpbms6aHJlZj0iI2hvbGUiIHRyYW5zZm9ybT0icm90YXRlKDE0NCkiLz4KICAgIDx1c2UgeGxpbms6aHJlZj0iI2hvbGUiIHRyYW5zZm9ybT0icm90YXRlKDIxNikiLz4KICAgIDx1c2UgeGxpbms6aHJlZj0iI2hvbGUiIHRyYW5zZm9ybT0icm90YXRlKDI4OCkiLz4KICA8L21hc2s+CjwvZz4KPC9zdmc+",
                                            "size": "Stretch",
                                            "height": "45px",
                                            "horizontalAlignment": "Left",
                                            "$when": "${$host.hostTheme==\"light\"}",
                                            "width": "45px"
                                        }
                                    ]
                                }
                            ]
                        }
                    ]
                }
            ],
            "$schema": "http://adaptivecards.io/schemas/adaptive-card.json",
            "version": "1.6",
            "verticalContentAlignment": "Top",
            "actions": [
                {
                    "type": "Action.Execute",
                    "title": "Increment",
                    "verb": "inc"
                }
            ]
        }"##))?;

        WidgetManager::GetDefault()?.UpdateWidget(&options)
    }
}

impl IWidgetProvider_Impl for WidgetProvider {
    fn CreateWidget(&self, widgetcontext: &core::option::Option<WidgetContext>) -> windows::core::Result<()> {
        let id = widgetcontext.as_ref().unwrap().Id()?;
        self.update_widget_template(&id)?;
        self.update_widget_data(&id)
    }

    fn DeleteWidget(&self, _widgetid: &windows::core::HSTRING, _customstate: &windows::core::HSTRING) -> windows::core::Result<()> {
        Ok(())
    }

    fn OnActionInvoked(&self, actioninvokedargs: &core::option::Option<WidgetActionInvokedArgs>) -> windows::core::Result<()> {
        if let Some(args) = actioninvokedargs {
            if args.Verb()? == "inc" {
                self.click_count.fetch_add(1, Ordering::Relaxed);
                self.update_widget_data(&args.WidgetContext()?.Id()?)?;
            }
        }
        Ok(())
    }

    fn OnWidgetContextChanged(&self, _contextchangedargs: &core::option::Option<WidgetContextChangedArgs>) -> windows::core::Result<()> {
        Ok(())
    }

    fn Activate(&self, _widgetcontext: &core::option::Option<WidgetContext>) -> windows::core::Result<()> {
        Ok(())
    }

    fn Deactivate(&self, _widgetid: &windows::core::HSTRING) -> windows::core::Result<()> {
        Ok(())
    }
}
