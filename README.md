# LinksChecker
<img src="https://img.shields.io/github/workflow/status/shree675/LinksChecker/Rust"></img>

*lync* for links

## About
Lync is a CLI tool that can be used for checking the status of external and internal links.  
Displays the status in a simplistic manner with basic functionality.

## Usage
```powershell
$ lync [OPTIONS]
```

Options:
```
  -s, --single <SITE>     Checks a single website
  -m, --multiple <SITES>  Checks multiple websites (comma separated)
  -d, --detail            Adds more details to the response
  -V, --version           Print version information
```

For more information, run:
```powershell
$ lync --help
```

<details>
<summary>Examples</summary>

```shell
$ lync -s https://www.google.com

Site 'https://www.google.com' pinged,
status: 200 OK,
completed in 0.2820669s
------------------
```

```shell
$ lync -d -m https://www.google.com,https://www.boxrec.com/,https://www.amazon.com,www.abc.com

Host: DESKTOP-LO1E3AV

For site 'www.abc.com',
builder error: relative URL without a base
------------------
Site 'https://www.boxrec.com/' pinged,
status: 403 Forbidden,
IPv4: NA,
completed in 0.08167569s
------------------
Site 'https://www.google.com' pinged,
status: 200 OK,
IPv4: 142.250.182.68,
completed in 0.394853s
------------------
Site 'https://www.amazon.com' pinged,
status: 200 OK,
IPv4: 18.155.91.129,
completed in 0.6934601s
------------------

```

</details>

## Features
* A robust error handling mechanism.
* Pings websites in parallel by launching threads when `-m` flag is used.
* Displays minimal information in a color coded fashion. More information is printed when `-d` flag is used.
* Light-weight and open for feature additions.

## Version
v0.1.0
