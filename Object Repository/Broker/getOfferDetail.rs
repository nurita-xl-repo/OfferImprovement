<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getOfferDetail</name>
   <tag></tag>
   <elementGuidId>1cccb341-139a-47c9-8321-a1ab0e8ed9f0</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>6b382a89-4362-4f9a-bb37-86deb7616fc0</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://10.23.40.58:9321/get_offers_detail?SUBSCRIBER_ID=${subs_id_purchase}&amp;CHANNEL_NAME=${chnl_nameOI}&amp;LOCATION=${loc_purchase}&amp;CGI_NAME=${cgi_default}&amp;OFFER_ID=159020</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.subs_id_purchase</defaultValue>
      <description></description>
      <id>8991f4d8-7b22-44f8-b380-182b002cf57e</id>
      <masked>false</masked>
      <name>subs_id_purchase</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.chnl_nameOI</defaultValue>
      <description></description>
      <id>6df9de0a-055b-48a7-930c-592e653c8ecd</id>
      <masked>false</masked>
      <name>chnl_nameOI</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.loc_purchase</defaultValue>
      <description></description>
      <id>8833a1da-5c4c-4305-b59d-a2642f376df3</id>
      <masked>false</masked>
      <name>loc_purchase</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cgi_default</defaultValue>
      <description></description>
      <id>2c3c809b-644b-41c4-980f-1844b009405e</id>
      <masked>false</masked>
      <name>cgi_default</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>