test: ssh/pubkeys
	docker compose up -d

ssh/pubkeys: ssh/id_rsa
	mkdir -v $@
	cp -v $(addsuffix .pub,$^) $@

ssh/id_rsa:
	mkdir -v $(@D)
	ssh-keygen -N '' -t rsa -f $@ -C "Key for testing chrclone"

clean:
	docker compose down -v
	$(RM) -rv ssh
