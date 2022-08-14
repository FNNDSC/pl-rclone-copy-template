DOCKER = docker
DOCKER_COMPOSE = docker compose

test: test-data

test-data: image test-server
	$(DOCKER_COMPOSE) exec -u root openssh-server \
		sh -c 'mkdir -p /neuro && chmod 777 /neuro'
	$(DOCKER) run --rm --net=host \
		-v $(PWD)/expected:/data/expected:ro \
		localhost/fnndsc/pl-rclone:testing \
		rclone sync -v /data/expected test-ssh-server:/neuro/example_data

test-server: ssh/pubkeys ssh/rclone.conf image
	$(DOCKER_COMPOSE) up -d

image: ssh/rclone.conf
	env DOCKER_BUILDKIT=1 $(DOCKER) build \
		-t localhost/fnndsc/pl-rclone:testing \
		--build-arg RCLONE_CONFIG_BASE64="`base64 < $<`" ..

ssh/rclone.conf: ssh/id_rsa
	./create_config.sh < $< > $@

ssh/pubkeys: ssh/id_rsa
	mkdir -v $@
	cp -v $(addsuffix .pub,$^) $@

ssh/id_rsa:
	mkdir -v $(@D)
	ssh-keygen -N '' -t rsa -f $@ -C "Key for testing chrclone"

clean:
	$(DOCKER_COMPOSE) down -v
	$(RM) -rv ssh


.PHONY: clean
