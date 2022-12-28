
// import * as datahike from "./datahike.darwin-x64";
const datahike = require('./datahike.darwin-arm64.node')


const main = async () => {
    console.log(await datahike.init());


    const config = "{:store {:backend :file :path \"./path-to-db\"} :schema-flexibility :read}";

    const exists = await datahike.databaseExists(config);
    console.log(exists);


    if (!exists) {
        console.log(await datahike.createDatabase(config));
    }

    let tx_data = "[{:age 42}]"

    console.log(await datahike.transact(config, tx_data));



}


main().then(console.log)
