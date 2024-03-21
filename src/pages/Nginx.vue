<script setup lang="ts">
import { ports } from "../enum/service.ts";
</script>

<template>
  <h2 class="text-2xl text-center my-5">Nginx Config</h2>
  <div class="bg-gray-100 p-5 rounded-lg">
    <code >
      <pre>
server
{
	listen 80;

	server_name ~^(?{{"<app>"}}.+)\.dokan\.cloud;

	client_max_body_size 200M;

	location /
	{
		include proxy_params;
{{ Object.keys(ports).map((key) => {
          return `\t \t if ($app = "${key.replace('-service', '')}") { proxy_pass http://localhost:${ports[key]};}`
        }).join("\n") }}
	}

}
      </pre>
    </code>
  </div>
</template>
