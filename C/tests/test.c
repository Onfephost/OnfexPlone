#include <stdio.h>
#include <string.h>

int main() {
    char username[20];
    char password[20];
    const char *correct_user = "admin";
    const char *correct_pass = "1234";
    int attempts = 3;
    
    printf("=== Login ===\n\n");
    
    while (attempts > 0) {
        printf("Username: ");
        fgets(username, sizeof(username), stdin);
        username[strcspn(username, "\n")] = 0;
        
        printf("Password: ");
        fgets(password, sizeof(password), stdin);
        password[strcspn(password, "\n")] = 0;
        
        if (strcmp(username, correct_user) == 0 && strcmp(password, correct_pass) == 0) {
            printf("\n✓ Logined up,Wellcome %s\n", username);
            return 0;
        } else {
            attempts--;
            if (attempts > 0) {
                printf("✗ Wrong username or password! Remaining attempts: %d\n\n", attempts);
            } else {
                printf("✗ Account locked! Too many failed attempts.\n");
            }
        }
    }
    
    return 1;
}
