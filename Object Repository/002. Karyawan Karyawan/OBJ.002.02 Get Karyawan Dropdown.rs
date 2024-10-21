<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OBJ.002.02 Get Karyawan Dropdown</name>
   <tag></tag>
   <elementGuidId>b86e5af2-8afb-40ef-a8a2-8009819facfc</elementGuidId>
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
  &quot;text&quot;: &quot;{\n  \&quot;fullName\&quot;: \&quot;Leo Messi\&quot;,\n  \&quot;nickName\&quot;: \&quot;Leo\&quot;,\n  \&quot;employeeType\&quot;: \&quot;Magang\&quot;,\n  \&quot;division\&quot;: \&quot;670781dd57c82a89d3b4e721\&quot;,\n  \&quot;position\&quot;: \&quot;6707824a57c82a89d3b4e727\&quot;,\n  \&quot;email\&quot;: \&quot;6708d0e875ed013ff0e7cccc\&quot;,\n  \&quot;phoneNumber\&quot;: \&quot;+62 85810017800\&quot;,\n  \&quot;nik\&quot;: \&quot;010101010101\&quot;,\n  \&quot;address\&quot;: \&quot;Jl Barcelona\&quot;,\n  \&quot;startWorkDate\&quot;: 1234567898765,\n  \&quot;endWorkDate\&quot;: 1234567898766,\n  \&quot;maritalStatus\&quot;: \&quot;Menikah\&quot;,\n  \&quot;totalDependents\&quot;: 1,\n  \&quot;accountNumber\&quot;: \&quot;103019\&quot;,\n  \&quot;accountNumberHolder\&quot;: \&quot;301910\&quot;,\n  \&quot;npwp\&quot;: \&quot;100110011001\&quot;,\n  \&quot;preset\&quot;: \&quot;6707838f57c82a89d3b4e730\&quot;,\n  \&quot;accessModule\&quot;: [\&quot;Hiring\&quot;]\n}&quot;,
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
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${karyawan}${endpoint}</restUrl>
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
      <defaultValue>GlobalVariable.karyawan</defaultValue>
      <description></description>
      <id>e4dc6ce3-e78c-45c8-a926-d516bc90bf9e</id>
      <masked>false</masked>
      <name>karyawan</name>
   </variables>
   <variables>
      <defaultValue>'/v1/employees/dropdown'</defaultValue>
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
