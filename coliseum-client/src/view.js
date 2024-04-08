const net = require('node:net');
const client = new net.Socket();

async function client_request(port, ip_address, request) {
    return new Promise((resolve, reject) => {
        client.connect(port, ip_address, () => {
            console.log('Conectado al servidor');
            client.write(JSON.stringify(request));
        });


        client.on('data', (response) => {
            console.log(response);
            client.end();
            resolve(response);
        });


        client.on('error', (error) => {
            console.error('An error occurred: ' + error.message);
            client.end();
            reject(error);
        });


        client.on('close', () => {
            console.log('Connection closed with:' + ip_address + ":" + port);
        });
    });
}

function display_transactions(transactions) {

    var transactions_array = JSON.parse(transactions);


    transactions_array.forEach(transaction => {

        let from = transaction.from;
        let to = transaction.to;
        let amount = transaction.amount;
        let date = transaction.date;
        let status = transaction.status;

        let table = document.querySelector("#transactions table tbody");
        let newRow = table.insertRow();
        newRow.innerHTML = `<td>${from}</td>
                            <td>${to}</td>
                            <td>${amount}</td>
                            <td>${date}</td>
                            <td>${status}</td>
                            <td>${comments}</td>`;
    });
}

async function get_account_information() {
    let nodeIp = document.getElementById("node_ip").value;
    let splitNodeIp = nodeIp.split(":");
    let ip_address = splitNodeIp[0];
    let port = splitNodeIp[1];
    let userName = document.getElementById("user_name").value;

    let request = {
        entity: "CLIENT",
        action: "GET_ACCOUNT",
        data: { user_name: userName }
    };

    try {
        const response = JSON.parse(await client_request(port, ip_address, request));
        const data = JSON.parse(response.data);
        document.getElementById("user_id").value = data.id;
        document.getElementById("account_balance").value = data.balance;
        display_transactions(data.transactions);
    } catch (error) {
        console.error('Error:', error);
    }
}

async function create_transaction() {
    let nodeIp = document.getElementById("node_ip").value;
    let splitNodeIp = nodeIp.split(":");
    let ip_address = splitNodeIp[0];
    let port = splitNodeIp[1];

    let account_id = document.getElementById("user_id").value;
    let transaction_from = document.getElementById("user_name").value;
    let transaction_to = document.getElementById("new_transaction_to").value;
    let transaction_amount = document.getElementById("new_transaction_amount").value;

    let data = {
        account_id: account_id,
        from: transaction_from,
        to: transaction_to,
        amount: transaction_amount
    };

    let request = {
        entity: "CLIENT",
        action: "CREATE_TRANSACTION",
        data: data
    };

    try {
        const response = JSON.parse(await client_request(port, ip_address, request));
        console.log(response);
    } catch (error) {
        console.error('Error:', error);
    }
}

document.getElementById("log_in_botton").addEventListener("click", get_account_information);
document.getElementById("new_transaction").addEventListener("click", create_transaction);
