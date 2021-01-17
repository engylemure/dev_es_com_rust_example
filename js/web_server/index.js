const express = require('express')
const beforeExit = require('before-exit')
const cluster = require('cluster')
const os = require('os')
const {
    SHUTDOWN_TIMEOUT = 10 * 1000,
    CLUSTER_MODE = 'true'
} = process.env
const port = process.env["SERVER_PORT"] ?? 3000

if (cluster.isMaster && CLUSTER_MODE === 'true') {
    const cpuCount = os.cpus().length

    for (let i = 0; i < cpuCount; i++) {
        cluster.fork()
    }

    beforeExit.do((signal) => {
        Object.values(cluster.workers).map(worker => worker.kill(signal))
        return new Promise((resolve, reject) => {
            setTimeout(() => {
                resolve()
            }, SHUTDOWN_TIMEOUT)
        })
    })
}
else {
    const app = express()
    app.get('/', (req, res) => {
        res.send('Hello Dev-ES Community!!')
    })
    app.get('/sieve/:number', (req, res) => {
        res.send(sieve(req.params.number))
    })
    app.listen(port, () => {
        console.log(`Servidor iniciado em http://localhost:${port}`)
    })
    beforeExit.do((signal) => {
        console.log(`Shutting down the server due signal '${signal}'...`)
    });
}


function sieve(number) {
    number = Number(number);
    let upper_bound = number + 1;
    let is_prime = new Array(upper_bound);
    is_prime.fill(true)
    for (let i = 2; i < upper_bound / 2; i++) {
        if (is_prime[i]) {
            let j = i * 2;
            while (j < upper_bound) {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    return is_prime.reduce((all_prime_numbers, is_prime, number) => {
        if (is_prime) {
            all_prime_numbers.push(number)
        }
        return all_prime_numbers
    }, [])
}