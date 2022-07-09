#!/usr/bin/env zx

const PATH_CLIENTS = './clients'

/**
 * General utils.
 */
const Utils = {
    parseInt: str => typeof str === 'Number' ? str : !!str ? parseInt(str) : str,
    exit: code => process.exit(code),
}

/**
 * Logging utils.
 */
const Log = {
    info: text => console.log(`${chalk.yellow('INFO')} ${text}`),
    out: filename => console.log(`${chalk.yellow('OUT')} ${chalk.cyan(filename)}`),
    example: (command, extra) => console.log(`${chalk.yellow('INFO')} Run ${chalk.green(command)} ${extra}`.trim())
}

/**
 * Print the help text.
 */
const printHelp = () => {
    console.log(`
        Usage: ./rescripts <subcommand> [options]

        subcommands:
        | version           Fetch latest FlyffU client version
        | dump              Dump and decompile latest FlyffU client

        options:
        | -h  --help        Show this help text
    `.trim().split('\n').map(x => x.trim()).map(x => x.startsWith('|') ? `  ${x.substring(1)}` : x).join('\n'))
}

class LocalClient {
    /**
     * Construct a new `LocalClient`.
     *
     * @param {string|number} version
     * @param {string} path
     */
    constructor(version, path) {
        this.version = Utils.parseInt(version)
        this.path = path
    }

    /**
     * Get a human-readable version string.
     *
     * @example
     * ```js
     * new LocalClient(42, './clients/v42').versionString() // 'v42'
     * ```
     */
    get versionString() {
        return `v${this.version}`
    }
}

/**
 * Extract current FilemapVersion from FlyffU website.
 *
 * @returns {number}
 */
const check_filemap_version = async () => {
    Log.info('Retrieving filemap version...')
    const url = 'https://universe.flyff.com/play'
    const resp = await fetch(url, { cache: 'no-cache' })
    const text = await resp.text()
    const re_version = /var\s+FilemapVersion\s*=\s*'(?<version>\d+)'/
    return Utils.parseInt(text.match(re_version).groups?.version)
}

/**
 * Dump main wasm binary.
 *
 * @param {number} version
 * @returns {Buffer} wasm data
 */
const dump_wasm = async version => {
    Log.info('Dumping main-wasm32...')
    const url = `https://gcpcdn-universe.flyff.com/client/program/web/main-wasm32.wasm?${version}`
    const resp = await fetch(url, { cache: 'no-cache' })
    const data = await resp.arrayBuffer()
    return Buffer.from(data)
}

/**
 * Enumerate local client versions.
 *
 * @returns {Array<LocalClient>}
 */
const enumerate_local_clients = async () => {
    Log.info('Checking for local client versions...')
    await fs.mkdirp(PATH_CLIENTS)
    const folders = await fs.readdir(PATH_CLIENTS)
    const clients = []
    for (const folder of folders.filter(p => p.startsWith('v'))) {
        const re_version = /^v(?<version>\d+)$/
        const version = folder.match(re_version).groups?.version
        if (version) {
            const client = new LocalClient(version, path.join(PATH_CLIENTS, folder));
            clients.push(client)
        }
    }
    clients.sort((a, b) => a.version < b.version)
    return clients
}

/**
 * Convert the main-wasm32.wasm to wat format
 *
 * @param {LocalClient} client
 */
const wasm2wat = async client => {
    Log.info('Translating wasm file into wat format...')
    const path_in = path.join(client.path, 'main-wasm32.wasm')
    const path_out = path.join(client.path, 'main-wasm32.wat')
    await $`wasm2wat ${path_in} -o ${path_out}`
    Log.out(path_out)
}

/**
 * Decompile the main-wasm32.wasm to into c
 *
 * @param {LocalClient} client
 */
 const wasm2c = async client => {
    Log.info('Decompiling wasm file to c. This will take a bit longer...')
    const path_in = path.join(client.path, 'main-wasm32.wasm')
    const path_out = path.join(client.path, 'main-wasm32.c')
    const path_out_h = path.join(client.path, 'main-wasm32.h')
    await $`wasm2c ${path_in} -o ${path_out}`
    Log.out(path_out)
    Log.out(path_out_h)
}

/**
 * Decompile the main-wasm32.wasm into a a more readable, c-like pseudo-language
 *
 * @param {LocalClient} client
 */
const wasm_decompile = async client => {
    Log.info('Decompiling wasm file into readable format...')
    const path_in = path.join(client.path, 'main-wasm32.wasm')
    const path_out = path.join(client.path, 'main-wasm32.pseudo.c')
    await $`wasm-decompile ${path_in} --o ${path_out}`
    Log.out(path_out)
}

/**
 * Main entry point.
 */
const main = async () => {
    // Check for -h and --help flags
    if (argv.h || argv.help) {
        printHelp()
        Utils.exit(0)
    }

    // Parse commands
    const args = argv._ ?? []
    switch (args.shift(0)) {

        // Check latest FlyffU client version
        case 'version': {
            // Check filemap version
            const version = await check_filemap_version()
            const local_clients = await enumerate_local_clients()
            const client = local_clients.find(client => client.version === version)
            if (client) {
                Log.info(`You are up to date (latest is v${version}).`)
            } else {
                const latest_local_version = local_clients.slice(-1)[0].version
                const diff = version - latest_local_version
                Log.info(`You are ${diff} version${diff === 1 ? '' : 's'} behind (local: v${latest_local_version}, latest: v${version}).`)
                Log.example('./rescripts dump', 'to grab the latest version.')
            }
            break
        }

        // Dump and decompile latest FlyffU client
        case 'dump': {
            // Check filemap version
            const version = await check_filemap_version()
            const local_clients = await enumerate_local_clients()
            {
                const client = local_clients.find(client => client.version === version)
                if (client) {
                    Log.info(`Client v${client.version} is already downloaded and located in ${client.path}`)
                    Utils.exit(0)
                }
            }
            Log.info(`Client v${version} not found`)
            const base_path = `${PATH_CLIENTS}/v${version}`
            const client = new LocalClient(version, base_path)
            {
                const wasm = await dump_wasm(version)
                const wasm_path = path.join(base_path, 'main-wasm32.wasm')
                await fs.mkdirp(base_path)
                await fs.writeFile(wasm_path, wasm)
                Log.out(wasm_path)
            }
            await wasm2wat(client)
            await wasm_decompile(client)
            await wasm2c(client)
            break
        }
        default: {
            printHelp()
            break
        }
    }
}

await main()
