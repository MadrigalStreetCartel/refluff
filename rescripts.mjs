#!/usr/bin/env zx

//
// Useful Constants
//

const PATH_CLIENTS = './clients'
const BASE_URL_CDN = 'https://gcpcdn-universe.flyff.com/client'

/**
 * Print the help text.
 */
 const printHelp = () => {
    console.log(`
        Usage: ./rescripts <subcommand> [options]

        subcommands:
        | version           Fetch latest FlyffU client version
        | dump              Dump and decompile latest FlyffU client
        | resdump           Dump known resources (world data, etc.)

        options:
        | -h  --help        Show this help text
    `.trim().split('\n').map(x => x.trim()).map(x => x.startsWith('|') ? `  ${x.substring(1)}` : x).join('\n'))
}

/**
 * General utils.
 */
const Utils = {
    exit: code => process.exit(code),
    parse_int: str => typeof str === 'Number' ? str : !!str ? parseInt(str) : str,
    fetch_binary: async (url, path) => {
        const resp = await fetch(url, { cache: 'no-cache' })
        const data = await resp.arrayBuffer()
        const buffer = Buffer.from(data)
        await fs.writeFile(path, buffer)
        Log.out(path)
    }
}

/**
 * Logging utils.
 */
const Log = {
    info: text => console.log(`${chalk.yellow('INFO')} ${text}`),
    out: filename => console.log(`${chalk.yellow('OUT')} ${chalk.cyan(filename)}`),
    progress: fraction => console.log(`${chalk.yellow('PROGRESS')} ${chalk.green((fraction * 100).toFixed(2))}%`),
    example: (command, extra) => console.log(`${chalk.yellow('INFO')} Run ${chalk.green(command)} ${extra}`.trim()),
}

class LocalClient {
    /**
     * Construct a new `LocalClient`.
     *
     * @param {string|number} version
     * @param {string} path
     */
    constructor(version, path) {
        this.version = Utils.parse_int(version)
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

class InclusiveRange {
    /**
     * Construct a new `InclusiveRange`.
     *
     * @param {number} start
     * @param {number} end
     */
    constructor(start, end) {
        this.start = Math.min(start, end)
        this.end = Math.max(start, end)
    }

    /**
     * Get the length of the range.
     *
     * @returns {number}
     */
    get length() {
        return 1 + this.end - this.start
    }

    /**
     * Offset the range start by `index`.
     *
     * @param {number} index
     * @returns {number}
     */
    offset(index) {
        return this.start + index
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
    return Utils.parse_int(text.match(re_version).groups?.version)
}

/**
 * Dump main wasm binary.
 *
 * @param {number} version
 * @returns {Buffer} wasm data
 */
const dump_wasm = async version => {
    Log.info('Dumping main-wasm32...')
    const url = `${BASE_URL_CDN}/program/web/main-wasm32.wasm?${version}`
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

const dump_world_data = async client => {
    Log.info('Dumping all wdmadrigal assets...')
    const base_dir = path.join(client.path, 'res', 'world', 'wdmadrigal')
    await fs.mkdirp(base_dir)
    await Utils.fetch_binary(`${BASE_URL_CDN}/world/wdmadrigal/wdmadrigal.bin`, path.join(base_dir, 'wdmadrigal.bin'))
    const range_i = new InclusiveRange(29, 36)
    const range_j = new InclusiveRange(9, 16)
    const total = range_i.length * range_j.length
    let count = 0
    for (let i = 0; i < range_i.length; i++) {
        const a = range_i.offset(i).toString().padStart(2, '0')
        for (let j = 0; j < range_j.length; j++) {
            Log.progress(count++ / total)
            const b = range_j.offset(j).toString().padStart(2, '0')
            const filename = `wdmadrigal${a}-${b}.bin`
            const url = `${BASE_URL_CDN}/world/wdmadrigal/${filename}`
            const out_path = path.join(base_dir, filename)
            await Utils.fetch_binary(url, out_path)
        }
    }
    Log.progress(1)
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

        // Dump known resources for the latest FlyffU client
        case 'resdump': {
            const version = await check_filemap_version()
            const local_clients = await enumerate_local_clients()
            const client = local_clients.find(client => client.version === version)
            if (!client) {
                Log.info(`Version ${version} has not been downloaded yet.`)
                Log.example('./rescripts dump', 'first, then rerun the resdump command.')
                Utils.exit(0)
            }
            await dump_world_data(client)
            break
        }
        default: {
            printHelp()
            break
        }
    }
}

await main()
