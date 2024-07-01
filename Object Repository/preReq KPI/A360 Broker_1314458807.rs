<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>A360 Broker_1314458807</name>
   <tag></tag>
   <elementGuidId>76c4c067-f5ef-43ba-a5f0-6e3e0e6d0b2a</elementGuidId>
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
   <restUrl>http://10.24.164.46:46000/entities/mccm_subscriber_profile?entity_id=1314458807&amp;subset_name=mccm_offer_drivers</restUrl>
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

def kpiArrayExpc = [85000, 15500, 80000, &quot;Xtra Combo VIP Plus&quot;,&quot;payu_segment_payu_users_outside_payu_norm_days&quot;,6,5,7,91];

def dslpck = kpiArrayExpc[8];
def avg = kpiArrayExpc[6];
def frd = kpiArrayExpc[0];
def usage = kpiArrayExpc[1];
def arpu = kpiArrayExpc[2];
def maxPayu = kpiArrayExpc[7];
def consecPayu = kpiArrayExpc[5];
def lastFamily = kpiArrayExpc[3];
def segment = kpiArrayExpc[4];

def result_dslpck = 'mccm_subscriber_profile_response.entity_payload.rt_kpis.data_days_since_last_pack_usage_3m';
def result_avg = 'mccm_subscriber_profile_response.entity_payload.rt_kpis.total_average_payu_days_3m_v2';
def result_frd = 'mccm_subscriber_profile_response.entity_payload.kpis.frd_reload_3m_v2';
def result_usage = 'mccm_subscriber_profile_response.entity_payload.kpis.max_usage_3m_v2';
def result_arpu = 'mccm_subscriber_profile_response.entity_payload.byop.max_arpu_90';
def result_consecPayu= 'mccm_subscriber_profile_response.entity_payload.rt_kpis.total_consecutive_payu_days_3m';
def result_lastFamily = 'mccm_subscriber_profile_response.entity_payload.rt_kpis.last_pack_product_family';
def result_segment = 'mccm_subscriber_profile_response.entity_segments.dynamic[0]';
def result_maxPayu = 'mccm_subscriber_profile_response.entity_payload.rt_kpis.total_max_payu_days_3m_v2';

WS.verifyElementPropertyValue(response, result_consecPayu, consecPayu)
WS.verifyElementPropertyValue(response, result_dslpck, dslpck)
WS.verifyElementPropertyValue(response, result_avg, avg)
WS.verifyElementPropertyValue(response, result_frd, frd)
WS.verifyElementPropertyValue(response, result_usage, usage)
WS.verifyElementPropertyValue(response, result_arpu, arpu)
WS.verifyElementPropertyValue(response, result_lastFamily, lastFamily)
WS.verifyElementPropertyValue(response, result_segment, segment)
WS.verifyElementPropertyValue(response, result_maxPayu, maxPayu)


println &quot;total_consecutive_payu_days_3m: &quot; + WS.getElementPropertyValue(response, result_consecPayu) + &quot; Expected: &quot;+ consecPayu
println &quot;total_average_payu_days_3m_v2: &quot; + WS.getElementPropertyValue(response, result_avg)+ &quot; Expected: &quot;+ avg
println &quot;total_max_payu_days_3m_v2: &quot; + WS.getElementPropertyValue(response, result_maxPayu) + &quot; Expected: &quot;+maxPayu
println &quot;data_days_since_last_pack_usage_3m: &quot; + WS.getElementPropertyValue(response, result_dslpck) + &quot; Expected: &quot;+dslpck
println &quot;frd_reload_3m_v2: &quot; + WS.getElementPropertyValue(response, result_frd)+&quot; Expected: &quot;+frd
println &quot;max_usage_3m_v2: &quot; + WS.getElementPropertyValue(response, result_usage)+ &quot; Expected: &quot;+usage
println &quot;max_arpu_90: &quot; + WS.getElementPropertyValue(response, result_arpu)+&quot; Expected: &quot;+arpu
println &quot;last_pack_product_family: &quot; + WS.getElementPropertyValue(response, result_segment)+&quot; Expected: &quot;+segment
println &quot;segment: &quot; + WS.getElementPropertyValue(response, result_lastFamily)+&quot; Expected: &quot;+lastFamily







</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
