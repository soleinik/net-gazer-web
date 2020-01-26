const Hapi = require('hapi');

const server = new Hapi.Server({
    port: 8080,
    host: 'localhost',
});


server.route({   
    method: 'GET',   
    path: '/',   
    handler: (request, reply) => {     
        return 'I am the home route'   
    } 
});




const launch = async () => {
    try { 
        await server.start(); 
    } catch (err) { 
        console.error(err); 
        process.exit(1); 
    }; 
    console.log(`Server running at ${server.info.uri}`); 
}

launch();
