<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>A360 Broker_484712694</name>
   <tag></tag>
   <elementGuidId>6d4384c3-76ea-41d6-a27d-9c63e1641545</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://10.24.164.46:46000/entities/mccm_subscriber_profile?entity_id=484712694&amp;subset_name=mccm_offer_drivers</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

def dslpck = 15;
def avg = 3;
def frd = 62000;
def usage = 9500;
def arpu = 57500;
def totalPackPurchase = 0;
def payu_revenue = null;
def lastFamily = 'Xtra Combo Plus';
def segment = 'payu_segment_payu_users_outside_payu_max_days';

def result_dslpck = 'mccm_subscriber_profile_response.entity_payload.rt_kpis.data_days_since_last_pack_usage_3m';
def result_avg = 'mccm_subscriber_profile_response.entity_payload.rt_kpis.total_average_payu_days_3m_v2';
def result_frd = 'mccm_subscriber_profile_response.entity_payload.kpis.frd_reload_3m_v2';
def result_usage = 'mccm_subscriber_profile_response.entity_payload.kpis.max_usage_3m_v2';
def result_arpu = 'mccm_subscriber_profile_response.entity_payload.byop.max_arpu_90';
def result_totalPurchase = 'mccm_subscriber_profile_response.entity_payload.rt_kpis.total_pack_purchase_amount_3m';
def result_payuRevenue = 'mccm_subscriber_profile_response.entity_payload.rt_kpis.total_payu_revenue_v3';
def result_lastFamily = 'mccm_subscriber_profile_response.entity_payload.rt_kpis.last_pack_product_family';
def result_segment = 'mccm_subscriber_profile_response.entity_segments.dynamic[0]';

WS.verifyElementPropertyValue(response, result_dslpck, dslpck)
WS.verifyElementPropertyValue(response, result_avg, avg)
WS.verifyElementPropertyValue(response, result_frd, frd)
WS.verifyElementPropertyValue(response, result_usage, usage)
WS.verifyElementPropertyValue(response, result_arpu, arpu)
WS.verifyElementPropertyValue(response, result_totalPurchase, totalPackPurchase)
WS.verifyElementPropertyValue(response, result_payuRevenue, payu_revenue)
WS.verifyElementPropertyValue(response, result_lastFamily, lastFamily)
WS.verifyElementPropertyValue(response, result_segment, segment)

println &quot;data_days_since_last_pack_usage_3m: &quot; + WS.getElementPropertyValue(response, result_dslpck)
println &quot;total_average_payu_days_3m_v2: &quot; + WS.getElementPropertyValue(response, result_avg)
println &quot;frd_reload_3m_v2: &quot; + WS.getElementPropertyValue(response, result_frd)
println &quot;max_usage_3m_v2: &quot; + WS.getElementPropertyValue(response, result_usage)
println &quot;max_arpu_90: &quot; + WS.getElementPropertyValue(response, result_arpu)
println &quot;total_pack_purchase_amount_3m: &quot; + WS.getElementPropertyValue(response, result_totalPurchase)
println &quot;total_payu_revenue_v3: &quot; + WS.getElementPropertyValue(response, result_payuRevenue)
println &quot;segment: &quot; + WS.getElementPropertyValue(response, result_lastFamily)
println &quot;last_pack_product_family: &quot; + WS.getElementPropertyValue(response, result_segment)




</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
