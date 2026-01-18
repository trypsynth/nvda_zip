# nvda_zip
This is a simple Actix-web application that runs at [nvda.zip](https://nvda.zip), providing an API to download the latest version of the NVDA screen reader.

## Endpoints:
* `/`: Download the latest stable NVDA version.
* `/stable.json`: Get a JSON response containing stable NVDA's direct download link and hash.
* `/xp`: Download the last NVDA version to run on Windows XP.
* `/xp.json`: Get a JSON response containing a direct download link to the last version of NVDA that ran on Windows XP and its launcher hash.
* `/win7`: Download the last NVDA version to run on Windows 7 Service Pack 1 and Windows 8.0.
* `/win7.json`: Get a JSON response containing a direct download link to the last version of NVDA that ran on Windows 7 Service Pack 1 and Windows 8.0 and its launcher hash.
* `/alpha`: Download the latest NVDA snapshot (alpha) version.
* `/alpha.json`: Get a JSON response containing a direct download link for the latest NVDA snapshot and its launcher hash.
* `/beta`: Download the latest NVDA beta version.
* `/beta.json`: Get a JSON response containing the latest NVDA beta's direct download link and launcher hash.

## License
This project is licensed under the [MIT license](license.md).
