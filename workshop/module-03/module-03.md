## Module III - Building the Dependent Modules

Before we begin constructing our first RESTful service for the DaaS Pattern, we need to first review the architecture overview of the DaaS Pattern. We see in the pattern that our DaaS services interact with a **database**, (we will be using CouchDB) and a **broker** (we will be using Kafka). There is also a **data pattern**, (a metadata data model) that causes the DaaS pPattern to become a reactive architecture. 

![DaaS Pattern](daas-pattern.png)