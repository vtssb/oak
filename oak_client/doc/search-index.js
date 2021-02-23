var searchIndex = JSON.parse('{\
"oak_client":{"doc":"Helper library for connecting to Oak from clients.","i":[[5,"create_tls_channel","oak_client","Creates a TLS channel for connecting to Oak.",null,[[["uri",3]]]],[0,"interceptors","","",null,null],[3,"CombinedInterceptor","oak_client::interceptors","This struct is created by the [`combine`] method. See its…",null,null],[5,"combine","","Combines the two provided interceptor, executing the first…",null,[[["interceptor",8]],[["combinedinterceptor",3],["interceptor",8]]]],[0,"auth","","",null,null],[3,"AuthInterceptor","oak_client::interceptors::auth","Intercepts gRPC requests and authenticates the client with…",null,null],[11,"create","","",0,[[["keypair",3]]]],[0,"label","oak_client::interceptors","",null,null],[3,"LabelInterceptor","oak_client::interceptors::label","Intercepts gRPC requests and adds the same `label` as a…",null,null],[11,"create","","",1,[[["label",3]],["result",6]]],[8,"Interceptor","oak_client::interceptors","Trait for interceptors, to facilitate combining until…",null,null],[10,"process","","",2,[[["request",3]],[["result",4],["request",3],["status",3]]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"into_request","","",3,[[],["request",3]]],[11,"vzip","","",3,[[]]],[11,"from","oak_client::interceptors::auth","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"into_request","","",0,[[],["request",3]]],[11,"vzip","","",0,[[]]],[11,"from","oak_client::interceptors::label","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"into_request","","",1,[[],["request",3]]],[11,"vzip","","",1,[[]]],[11,"process","oak_client::interceptors::auth","",0,[[["request",3]],[["result",4],["request",3],["status",3]]]],[11,"process","oak_client::interceptors::label","",1,[[["request",3]],[["result",4],["request",3],["status",3]]]],[11,"process","oak_client::interceptors","",3,[[["request",3]],[["result",4],["request",3],["status",3]]]],[11,"into","oak_client::interceptors::auth","",0,[[],["interceptor",3]]],[11,"into","oak_client::interceptors::label","",1,[[],["interceptor",3]]],[11,"into","oak_client::interceptors","",3,[[],["interceptor",3]]]],"p":[[3,"AuthInterceptor"],[3,"LabelInterceptor"],[8,"Interceptor"],[3,"CombinedInterceptor"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);