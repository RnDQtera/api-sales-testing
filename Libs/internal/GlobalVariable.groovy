package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.main.TestCaseMain


/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p></p>
     */
    public static Object token
     
    /**
     * <p></p>
     */
    public static Object karyawan_settings
     
    /**
     * <p></p>
     */
    public static Object custom_field
     
    /**
     * <p></p>
     */
    public static Object karyawan
     
    /**
     * <p></p>
     */
    public static Object expired_terms
     
    /**
     * <p></p>
     */
    public static Object settings_user
     
    /**
     * <p></p>
     */
    public static Object sales_approval
     
    /**
     * <p></p>
     */
    public static Object settings_role
     
    /**
     * <p></p>
     */
    public static Object customer_partner
     
    /**
     * <p></p>
     */
    public static Object crm_product
     

    static {
        try {
            def selectedVariables = TestCaseMain.getGlobalVariables("default")
			selectedVariables += TestCaseMain.getGlobalVariables(RunConfiguration.getExecutionProfile())
            selectedVariables += TestCaseMain.getParsedValues(RunConfiguration.getOverridingParameters(), selectedVariables)
    
            token = selectedVariables['token']
            karyawan_settings = selectedVariables['karyawan_settings']
            custom_field = selectedVariables['custom_field']
            karyawan = selectedVariables['karyawan']
            expired_terms = selectedVariables['expired_terms']
            settings_user = selectedVariables['settings_user']
            sales_approval = selectedVariables['sales_approval']
            settings_role = selectedVariables['settings_role']
            customer_partner = selectedVariables['customer_partner']
            crm_product = selectedVariables['crm_product']
            
        } catch (Exception e) {
            TestCaseMain.logGlobalVariableError(e)
        }
    }
}
