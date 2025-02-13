(ns onekeepass.frontend.constants)

;; This is the default Entry type to use 
(def UUID_OF_ENTRY_TYPE_LOGIN "ffef5f51-7efc-4373-9eb5-382d5b501768")

;; Standard Entry Type Names
;; These should match names used in 'standard_entry_types.rs'

(def LOGIN_TYPE_NAME "Login")
(def CREDIT_DEBIT_CARD_TYPE_NAME "Credit/Debit Card")
(def WIRELESS_ROUTER_TYPE_NAME "Wireless Router")
(def PASSPORT_TYPE_NAME "Passport")
(def BANK_ACCOUNT_TYPE_NAME "Bank Account")
;;
(def CATEGORY_ALL_ENTRIES "AllEntries")
(def CATEGORY_FAV_ENTRIES "Favorites")
(def CATEGORY_DELETED_ENTRIES "Deleted")

#_(def GENERAL_CATEGORIES #{CATEGORY_ALL_ENTRIES CATEGORY_FAV_ENTRIES CATEGORY_DELETED_ENTRIES})

(def MENU_ID_QUIT "Quit")
(def MENU_ID_SEARCH "Search")
(def MENU_ID_NEW_DATABASE "NewDatabase")
(def MENU_ID_SAVE_DATABASE "SaveDatabase")
(def MENU_ID_OPEN_DATABASE "OpenDatabase")
(def MENU_ID_LOCK_DATABASE "LockDatabase")
(def MENU_ID_CLOSE_DATABASE "CloseDatabase")
(def MENU_ID_PASSWORD_GENERATOR "PasswordGenerator")
#_(def MENU_ID_NEW_ENTRY "NewEntry")
(def MENU_ID_EDIT_ENTRY "EditEntry")

(def MENU_ENABLE "Enable")
(def MENU_DISABLE "Disable")

(def ADD_TAG_PREFIX "Add New Tag:")

(def DB_CHANGED "DbFileContentChangeDetected")

(def TOUCH_ID "TouchID")
(def FACE_ID "FaceID")
(def NO_BIOMETRIC "None")

(def PASSWORD "Password")
