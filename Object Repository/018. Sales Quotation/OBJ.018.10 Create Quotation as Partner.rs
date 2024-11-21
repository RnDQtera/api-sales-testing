<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OBJ.018.10 Create Quotation as Partner</name>
   <tag></tag>
   <elementGuidId>dcc07e63-18f2-43d1-9da3-d6032e772d9c</elementGuidId>
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
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;pipeline&quot;,
      &quot;value&quot;: &quot;671f2180d10851bf638fc412&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;partner&quot;,
      &quot;value&quot;: &quot;[67171df8df28bbc590be7cfc]&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;deliveryEstimation&quot;,
      &quot;value&quot;: &quot;[1,2]&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;attn&quot;,
      &quot;value&quot;: &quot;[\&quot;67171df8df28bbc590be7cfc\&quot;]&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;termAndConditions&quot;,
      &quot;value&quot;: &quot;[6712131061a50cb251386b7a]\n  \&quot;percentage\&quot;: 0,\n  \&quot;name\&quot;: \&quot;string\&quot;,\n  \&quot;purchaseDate\&quot;: \&quot;2024-11-21T00:00:40.868Z\&quot;,\n  \&quot;purchasePrice\&quot;: 0\n}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;termOfPayment&quot;,
      &quot;value&quot;: &quot;Kontan sebelum DO Barang&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;paymentStages&quot;,
      &quot;value&quot;: &quot;[{\n  \&quot;percentage\&quot;: 0,\n  \&quot;name\&quot;: \&quot;string\&quot;,\n  \&quot;purchaseDate\&quot;: \&quot;2024-11-21T00:00:40.868Z\&quot;,\n  \&quot;purchasePrice\&quot;: 0\n}]&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;qrf&quot;,
      &quot;value&quot;: &quot;6721b4208be8755771f71324&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;products&quot;,
      &quot;value&quot;: &quot;[{\n  \&quot;_id\&quot;: \&quot;67191a34efd07bde0251c902\&quot;,\n  \&quot;serviceValidity\&quot;: \&quot;string\&quot;,\n  \&quot;isVisible\&quot;: true,\n  \&quot;capitalPrice\&quot;: 0,\n  \&quot;sellingPrice\&quot;: 0,\n  \&quot;margin\&quot;: 0,\n  \&quot;distributorTOP\&quot;: {\n    \&quot;termOfPayment\&quot;: \&quot;Kontan sebelum DO Barang\&quot;,\n    \&quot;paymentStages\&quot;: [\n      {\n        \&quot;percentage\&quot;: 0,\n        \&quot;name\&quot;: \&quot;string\&quot;,\n        \&quot;purchaseDate\&quot;: \&quot;2024-11-21T00:00:40.868Z\&quot;,\n        \&quot;purchasePrice\&quot;: 0\n      }\n    ]\n  }\n}]&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;67191a34efd07bde0251c902&quot;,
      &quot;value&quot;: &quot;/Users/daffattrmdzi/Desktop/Screenshot 2024-11-21 at 10.24.11.png&quot;,
      &quot;type&quot;: &quot;File&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
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
      <webElementGuid>eb95c6ef-9b8d-48ac-b3f7-311f7b71ee5f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://dev-sales-api-quotation.asdf.id/v1/quotations/partners</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>4af89381-9ee1-49d3-97bb-e796d4610c49</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'/v1/sales/end-user-data-form/'</defaultValue>
      <description></description>
      <id>03d20122-a8d1-493d-966b-96fb4bf1a755</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.sales_end_user_data</defaultValue>
      <description></description>
      <id>e30a5c32-ed04-4be5-9ef0-de8f6fb8aff1</id>
      <masked>false</masked>
      <name>sales_end_user_data</name>
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
