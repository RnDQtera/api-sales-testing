<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OBJ.012.05 Edit Revision Re-submit SPRF</name>
   <tag></tag>
   <elementGuidId>7ccb4006-7c17-4394-9634-9a091b52b8a2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;pipelineId\&quot;: \&quot;67210250ffe74e4df7156e5c\&quot;,\n    \&quot;partnerId\&quot;: \&quot;6721aa80fddb8e69adf03d6b\&quot;,\n    \&quot;futureRevPotential\&quot;: \&quot;ASDF\&quot;,\n    \&quot;personInChargeIds\&quot;: [\&quot;6721aacdfddb8e69adf03d6f\&quot;],\n    \&quot;budgetSales\&quot;: 1000,\n    \&quot;justification\&quot;: \&quot;none\&quot;,\n    \&quot;products\&quot;: [\n        {\n            \&quot;id\&quot;: \&quot;671c8cfb4de3562d48cd78e1\&quot;,\n            \&quot;requestPrice\&quot;: 500,\n            \&quot;requestDiscount\&quot;: 30,\n            \&quot;freeQuantity\&quot;: 2,\n            \&quot;priceWithFreeUnit\&quot;: 10,\n            \&quot;discountWithFreeUnit\&quot;: 10\n        }\n    ],\n    \&quot;termOfPayment\&quot;: \&quot;Kontan sebelum DO Barang\&quot;,\n    \&quot;paymentStages\&quot;: [\n        {\n            \&quot;percentage\&quot;: 5,\n            \&quot;name\&quot;: \&quot;Dapo\&quot;,\n            \&quot;purchaseDate\&quot;: 10,     \n            \&quot;purchasePrice\&quot;: 5          \n        }\n    ],\n    \&quot;marginTotal\&quot;: 20\n}\n&quot;,
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
      <webElementGuid>f02040cb-b390-4101-85ce-19c5ed33bb47</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>8f6ee84e-2b63-499c-ab2b-f9e4ea8cab75</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>f335d01b-9bad-4ddd-a83a-725054773c06</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${sales_sprf}${endpoint}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>4af89381-9ee1-49d3-97bb-e796d4610c49</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sales_sprf</defaultValue>
      <description></description>
      <id>e4dc6ce3-e78c-45c8-a926-d516bc90bf9e</id>
      <masked>false</masked>
      <name>sales_sprf</name>
   </variables>
   <variables>
      <defaultValue>'v1/sprf/6721ab468be8755771f71254'</defaultValue>
      <description></description>
      <id>eb9dd16c-a05f-471e-8020-5202ebbb0a22</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
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
