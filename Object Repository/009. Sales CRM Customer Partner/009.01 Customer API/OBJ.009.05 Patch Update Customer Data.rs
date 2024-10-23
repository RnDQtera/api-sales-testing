<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OBJ.009.05 Patch Update Customer Data</name>
   <tag></tag>
   <elementGuidId>29fead12-c09d-422a-9b59-be86ef89674c</elementGuidId>
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
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;name&quot;,
      &quot;value&quot;: &quot;Bang Jago&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;accountManager&quot;,
      &quot;value&quot;: &quot;66c833f028754c4f75532f00&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;npwp&quot;,
      &quot;value&quot;: &quot;/Users/daffattrmdzi/Downloads/LOGO PPSI 1.png&quot;,
      &quot;type&quot;: &quot;File&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;pcib&quot;,
      &quot;value&quot;: &quot;2&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;address&quot;,
      &quot;value&quot;: &quot;Jl. Buntu&quot;,
      &quot;type&quot;: &quot;Text&quot;,
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
      <webElementGuid>ff0d836f-0119-4800-99e9-c4c0a6848ba6</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>f80c654f-36ba-40dd-bb2d-54441db92480</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>${customer_partner}${endpoint}</restUrl>
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
      <defaultValue>GlobalVariable.customer_partner</defaultValue>
      <description></description>
      <id>e4dc6ce3-e78c-45c8-a926-d516bc90bf9e</id>
      <masked>false</masked>
      <name>customer_partner</name>
   </variables>
   <variables>
      <defaultValue>'v1/customers/670f33c3ef3bfe68272fa9cc'</defaultValue>
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
