List of stuff to implement:
* auto register? - saves forms (pdf - might do png -> pdf)
* token registration on account creation
	- token format <#username sha128>.<#creation_timestamp>.<#lastname sha128>
	  example: 5X3%w2@9=xV.1700394843.2kFis&X

* token generation and authentication
* paypal integration w/ webhook <https://developer.paypal.com/api/rest/webhooks/>
* payment table - payment related things
* forms table - shows different forms and status
* events page (rows of events table)
* registration (table with rows)
* meet schedule table 
* inventory table
* trigger second lambda on email? for weekly updates
* read reciepts

Admin stuff:
* remove user
* add user
* lookup user
* view forms and payment
* add/remove events

token:
generate new token when new login
token is made of #timestamp.b64expires.#username
