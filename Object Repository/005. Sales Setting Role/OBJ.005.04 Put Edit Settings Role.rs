<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OBJ.005.04 Put Edit Settings Role</name>
   <tag></tag>
   <elementGuidId>5ca64e01-ad81-4622-8f6d-587be748b51f</elementGuidId>
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
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;:\&quot;Total Control\&quot;,\n    \&quot;description\&quot;:\&quot;Total Control\&quot;,\n    \&quot;isActive\&quot;:true,\n    \&quot;permissions\&quot;:{\n      \&quot;sales\&quot;:true,\n      \&quot;seniorSales\&quot;:true,\n      \&quot;admin\&quot;:true,\n      \&quot;elsa\&quot;:true,\n      \&quot;licenseTrial\&quot;:true,\n      \&quot;incentive\&quot;:true,\n      \&quot;priceInformation\&quot;:true,\n      \&quot;referenceNumber\&quot;:true,\n      \&quot;settingsUser\&quot;:true,\n      \&quot;settingsApproval\&quot;:true,\n      \&quot;settingsRole\&quot;:true,\n      \&quot;settingsExpiredTime\&quot;:true,\n      \&quot;settingsCustomField\&quot;:true,\n      \&quot;settingsTermAndCondition\&quot;:true\n  }\n}&quot;,
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
   <restUrl>${settings_role}${endpoint}</restUrl>
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
      <defaultValue>GlobalVariable.settings_role</defaultValue>
      <description></description>
      <id>e4dc6ce3-e78c-45c8-a926-d516bc90bf9e</id>
      <masked>false</masked>
      <name>settings_role</name>
   </variables>
   <variables>
      <defaultValue>'/v1/settings/roles/670ce1b2dbe87708ee67afc6'</defaultValue>
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
