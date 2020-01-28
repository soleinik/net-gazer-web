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

server.route({   
    method: 'GET',   
    path: '/chart',   
    handler: (request, reply) => {     
        return reply.file('./static/chart.html');
    } 
});

server.route({   
    method: 'GET',   
    path: '/chart/data.json',   
    handler: (request, reply) => {     
        return reply.file('./static/data.json');
    } 
});





async function launch () {  
    await server.register({
      plugin: require('inert')
    })
  
    await server.start()
    console.log('Server started at: ' + server.info.uri)
}
  
launch()



