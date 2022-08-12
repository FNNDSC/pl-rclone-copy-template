test: ssh/pubkeys ssh/rclone.conf
	docker compose up -d

ssh/rclone.conf: ssh/id_rsa
	./create_config.sh < $< > $@

ssh/pubkeys: ssh/id_rsa
	mkdir -v $@
	cp -v $(addsuffix .pub,$^) $@

ssh/id_rsa:
	mkdir -v $(@D)
	ssh-keygen -N '' -t rsa -f $@ -C "Key for testing chrclone"

clean:
	docker compose down -v
	$(RM) -rv ssh
