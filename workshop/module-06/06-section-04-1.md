### Section IV
>[CouchDB API](http://docs.couchdb.org/en/latest/api/index.html)

>[rust-daas.postman_collection.json](https://github.com/dsietz/rust-daas/blob/master/tests/rust-daas.postman_collection.json)

>[history.json](https://github.com/dsietz/rust-daas/blob/master/tests/database/_design/history.json)

---

When following a true Microservice architecture, it is important to remember that each microsservice has its own data source. This ensures higher availablity of the service and supports agility of the architecture by reduces inter-dependencies. All too often, the  microservice's application layer are constructed as independently executables, while their data source(s) remain tightly coupled.

Since the _Order Status History_ RESTful service only the consumer of the reporting data, (and _source of record satellite_ ) the database will be synchronized in realtime uni-directionally from the `provisioning` datbasase (_source of record_). 

##### Creating the Database
What's nice about CouchDB is that the platform doesn't require the installation of client software or driver. All database activities can either be done (1) manually through the Fauxton Console (http://couchdb-servcer:5984/_utils/) or (2) programmatically through CouchDB's RESTful API (http://docs.couchdb.org/en/latest/api/index.html).

Let's create a new databasse named `consuming` using the RESTful API. This database will only house a copy of the reporting data that's required for the _Order Status History_ RESTful service.

**HTTP Request**
```
curl -X PUT \
  'http://localhost:5984/consuming?q=8&n=3&partitioned=false&partitioned%20=false' \
  -H 'Accept: */*' \
  -H 'Content-type: application/json' \
  -H 'cache-control: no-cache'
```

**HTTP Response**
```
{
    "ok": true
}
```

##### Replicating the Realtime Data
Now that we have a database for the _Order Status History_ RESTful service, we will setup the uni-directional realtime sychronization.

**HTTP Request**
```
curl -X POST \
  http://localhost:5984/_replicate \
  -H 'Accept: application/json' \
  -H 'Accept-Encoding: gzip, deflate' \
  -H 'Authorization: Basic YWRtaW46cGFzc3dvcmQ=' \
  -H 'Cache-Control: no-cache' \
  -H 'Connection: keep-alive' \
  -H 'Content-Length: 88' \
  -H 'Content-Type: application/json' \
  -H 'Host: localhost:5984' \
  -H 'cache-control: no-cache' \
  -d '{
    "continuous" : true,
    "source": "provisioning",
    "target": "consuming"
}'
```

**HTTP Response**
```
{
    "ok": true,
    "_local_id": "bedfef67af6c3e82a425fece02aee9fc+continuous"
}
```

Just verify that the documents are now replicated to the `consuming` database, make a call to the CouchDB API to get a list of all the documents in the `consuming` databaase.

**HTTP Request**
```
curl -X GET \
  http://localhost:5984/consuming/_all_docs \
  -H 'Accept: application/json' \
  -H 'Accept-Encoding: gzip, deflate' \
  -H 'Authorization: Basic YWRtaW46cGFzc3dvcmQ=' \
  -H 'Cache-Control: no-cache' \
  -H 'Connection: keep-alive' \
  -H 'Host: localhost:5984' \
  -H 'cache-control: no-cache'
  ```

**HTTP Response**
```
{
    "total_rows": 2,
    "offset": 0,
    "rows": [
        {
            "id": "history|status|iStore|8003",
            "key": "history|status|iStore|8003",
            "value": {
                "rev": "3-9b8f57b145ddfea0e3fec0147a6cb835"
            }
        },
        {
            "id": "history|status|iStore|8004",
            "key": "history|status|iStore|8004",
            "value": {
                "rev": "1-1f4e25879ee846bc6ca4d5a00bbfb472"
            }
        }
    ]
}
```

##### Creating a Data View
Now that we have data in the `consuming` databaase, we should build a data interface (View) for the _Order Status History_ RESTful service to interact with to retreive the data. We will do this by creating a `_design` document using the CouchDB API

**HTTP Request**
```
curl -X PUT \
  http://localhost:5984/consuming/_design/history \
  -H 'Accept: application/json' \
  -H 'Accept-Encoding: gzip, deflate' \
  -H 'Authorization: Basic YWRtaW46cGFzc3dvcmQ=' \
  -H 'Cache-Control: no-cache' \
  -H 'Connection: keep-alive' \
  -H 'Content-Length: 504' \
  -H 'Content-Type: application/json' \
  -H 'Host: localhost:5984' \
  -H 'cache-control: no-cache' \
  -d '{
    "_id": "_design/history",
    "views": {
      "status-duration": {
        "map": "function (doc) {\n  \n  for(s = 0; s < doc.data_obj.order_status.length; s++) {\n    var duration = doc.data_obj.order_status[s].timestamp - doc.data_obj.timestamp;\n    emit(doc.data_obj.order_status[s].name, duration/60);\n  }\n}",
        "reduce": "function (keys, values, rereduce) {\n    avg = Math.round(sum(values)/values.length);\n    return avg;\n}"
      }
    },
    "language": "javascript"
}'
```

**HTTP Response**
```
{
    "ok": true,
    "id": "_design/history",
    "rev": "1-61a4a24d9e3a86cf154665a0588211ea"
}
```