<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Execution</name>
   <tag></tag>
   <elementGuidId>0630c253-bb9c-4e37-99e9-d1cedc0900ad</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjb250ZXh0Ijp7ImJhc2VVcmwiOiJodHRwczovL2JoYWd5ZXNoMTIzLmF0bGFzc2lhbi5uZXQiLCJ1c2VyIjp7ImFjY291bnRJZCI6IjYzNmI0Mzc1MGVmMjBhNWM0MGEzZDRlYyJ9fSwiaXNzIjoiY29tLmthbm9haC50ZXN0LW1hbmFnZXIiLCJzdWIiOiI1MmE5Nzk0MS1hMTkzLTM5YTItYTQ2Zi0yYWU3OThiZWRlNTYiLCJleHAiOjE3MTg1NDkxOTMsImlhdCI6MTY4NzAxMzE5M30.PiuINCr9EJO36ear71kTlTHhScFgrXw8wlX4GIzaubw</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;projectKey\&quot; : \&quot;${GlobalVariable.g_ZS_ProjectKey}\&quot;,\n\&quot;testCaseKey\&quot; : \&quot;${GlobalVariable.g_ZS_TestCaseKey}\&quot;,\n\&quot;testCycleKey\&quot; : \&quot;${GlobalVariable.g_ZS_TestCycleKey}\&quot;,\n\&quot;statusName\&quot; : \&quot;${GlobalVariable.g_ZS_StatusName}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>aa5915ee-4001-4fdb-a291-f1e9ad60be02</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjb250ZXh0Ijp7ImJhc2VVcmwiOiJodHRwczovL2JoYWd5ZXNoMTIzLmF0bGFzc2lhbi5uZXQiLCJ1c2VyIjp7ImFjY291bnRJZCI6IjYzNmI0Mzc1MGVmMjBhNWM0MGEzZDRlYyJ9fSwiaXNzIjoiY29tLmthbm9haC50ZXN0LW1hbmFnZXIiLCJzdWIiOiI1MmE5Nzk0MS1hMTkzLTM5YTItYTQ2Zi0yYWU3OThiZWRlNTYiLCJleHAiOjE3MTg1NDkxOTMsImlhdCI6MTY4NzAxMzE5M30.PiuINCr9EJO36ear71kTlTHhScFgrXw8wlX4GIzaubw</value>
      <webElementGuid>4af2735d-238d-4a50-8054-ee4cb0dbdfa5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api.zephyrscale.smartbear.com/v2/testexecutions</restUrl>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
