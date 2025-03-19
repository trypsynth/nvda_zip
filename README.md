# nvda_zip
This is a simple Actix-web application that runs at `https://nvda.zip`, providing an API to download the latest version of the NVDA screen reader.

## Endpoints:
* `/`: Download the latest stable NVDA version.
* `/stable.json`: Get a JSON response containing NVDA's current stable version number.
* `/xp`: Download the last NVDA version to run on Windows XP.
* `/xp.json`: Get a JSON response containing the last version of NVDA that ran on Windows XP.
* `/win7`: Download the last NVDA version to run on Windows 7 Service Pack 1 and Windows 8.0.
* `/win7.json`: Get a JSON response containing the last version of NVDA that ran on Windows 7 Service Pack 1 and Windows 8.0.
* `/alpha`: Download the latest NVDA snapshot (alpha) version.
* `/alpha.json`: Get a JSON response containing the latest alpha version number.
* `/beta`: Download the latest NVDA beta version.
* `/beta.json`: Get a JSON response containing the current NVDA beta version number.
