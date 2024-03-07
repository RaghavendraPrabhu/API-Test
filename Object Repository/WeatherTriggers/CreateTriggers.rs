<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateTriggers</name>
   <tag></tag>
   <elementGuidId>dd3b31ff-be29-46ab-bc18-447330ff1049</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \&quot;time_period\&quot;:{\n      \&quot;start\&quot;:{\n         \&quot;expression\&quot;:\&quot;after\&quot;,\n         \&quot;amount\&quot;:132000000\n      },\n      \&quot;end\&quot;:{\n         \&quot;expression\&quot;:\&quot;after\&quot;,\n         \&quot;amount\&quot;:432000000\n      }\n   },\n   \&quot;conditions\&quot;:[\n      {\n         \&quot;name\&quot;:\&quot;temp\&quot;,\n         \&quot;expression\&quot;:\&quot;$gt\&quot;,\n         \&quot;amount\&quot;:299\n      }\n   ],\n   \&quot;area\&quot;:[\n      {\n         \&quot;type\&quot;:\&quot;Point\&quot;,\n         \&quot;coordinates\&quot;:[\n            53,\n            37\n         ]\n      }\n   ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>19019f46-aa17-4cd1-9596-2e225b8b078f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.8</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://api.openweathermap.org/data/3.0/triggers?appid=1885c9beb33121d4f92a7b9420946951</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
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







WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
