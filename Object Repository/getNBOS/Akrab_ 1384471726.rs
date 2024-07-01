<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Akrab_ 1384471726</name>
   <tag></tag>
   <elementGuidId>e50e36cf-f945-4bf8-91fe-1a0aa86c666a</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
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
      <webElementGuid>eb249b32-36a3-48a2-bb6a-b15137908e9e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://10.23.40.58:9068/get_next_best_offers?SUBSCRIBER_ID=1384471726&amp;CHANNEL_NAME=MYXLULTIMATE_V2_XLPRE&amp;LOCATION=XL_Store*Section_7&amp;CGI_NAME=510-11-1-17681</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>e813d01e-9893-4712-aa8b-69f376feb139</id>
      <name>ValidateResponse</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>C:\Users\nurit\OneDrive - PT XL Axiata Tbk\03 Project Nurita\Katalon Automation Testing\Schema\getNBOS_schema.txt</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.sub_id_closestHigher</defaultValue>
      <description></description>
      <id>ab252dfd-3a46-45ca-b44a-66d08f2b7559</id>
      <masked>false</masked>
      <name>sub_id_closestHigher</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.chnl_nameOI</defaultValue>
      <description></description>
      <id>15baa12e-9c76-4e58-9f44-fc6811a72b1c</id>
      <masked>false</masked>
      <name>chnl_nameOI</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.loc_cH_Akrab</defaultValue>
      <description></description>
      <id>dfd482ca-1e1e-4a6d-abcf-b50c1befbba4</id>
      <masked>false</masked>
      <name>loc_cH_Akrab</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cgi_default</defaultValue>
      <description></description>
      <id>ada0dcca-2114-4fa6-9aa2-5d016bfb0769</id>
      <masked>false</masked>
      <name>cgi_default</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


def variables = request.getVariables()

WS.verifyElementPropertyValue(response, 'get_next_best_offer_response.sequence[1].channel_attribute[36].value', &quot;Akrab 25GB disc0% 110000&quot;)
WS.verifyElementPropertyValue(response, 'get_next_best_offer_response.sequence[2].channel_attribute[36].value', &quot;Akrab 45GB disc31% 125000&quot;)
WS.verifyElementPropertyValue(response, 'get_next_best_offer_response.sequence[3].channel_attribute[24].value', &quot;Akrab 80GB disc36% 180000&quot;)
WS.verifyElementPropertyValue(response, 'get_next_best_offer_response.sequence[4].channel_attribute[12].value', &quot;Akrab 160GB disc47% 240000&quot;)
WS.verifyElementPropertyValue(response, 'get_next_best_offer_response.sequence[5].channel_attribute[9].value', &quot;Akrab 16GB disc0% 77500&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
