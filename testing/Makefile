DOCKER = docker
DOCKER_COMPOSE = docker compose

IMAGE_NAME = localhost/fnndsc/pl-rclone:testing

test: test-ds test-fs

test-ds: image test-data
	./test_ds.sh

test-fs: image test-data
	./test_fs.sh

test-data: image test-server
	$(DOCKER_COMPOSE) exec -u root openssh-server \
		sh -c 'mkdir -p /neuro && chmod 777 /neuro'
	$(DOCKER) run --rm --net=host \
		-v $(PWD)/expected:/data/expected:ro \
		$(IMAGE_NAME) \
		rclone sync -v /data/expected test-ssh-server:/neuro/example_data

test-server: ssh/pubkeys ssh/rclone.conf image
	$(DOCKER_COMPOSE) up -d

image: ssh/rclone.conf
	env DOCKER_BUILDKIT=1 $(DOCKER) build \
		-t $(IMAGE_NAME) \
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
	$(DOCKER) rmi --force $(IMAGE_NAME)
	$(RM) -rv ssh


.PHONY: clean
