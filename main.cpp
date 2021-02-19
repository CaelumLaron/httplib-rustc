#include <cstring>
#include <iostream>
#include "rustc-http.h"

void hello_handler (const C_Request *req, C_Response *res) {
    res->body = strdup("Hello, world!");
}

void autocomplete_handler (const C_Request *req, C_Response *res) {
    res->body = strdup("Hello, autocomplete!");
}

int main() {
    const uint8_t ip_addr[] = {127,0,0,1};
    C_Server server;
    server.number_routes = 0;
    
    add_route(&server, "/lucas", hello_handler);
    add_route(&server, "/ajax/autocomplete/", autocomplete_handler);
    listen_at(&server, ip_addr,  9290);
}