#[doc = "Register `uhs_basic` reader"]
pub struct R(crate::R<UHS_BASIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHS_BASIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHS_BASIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHS_BASIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uhs_basic` writer"]
pub struct W(crate::W<UHS_BASIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHS_BASIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UHS_BASIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHS_BASIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_init_en` reader - "]
pub type REG_INIT_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_init_en` writer - "]
pub type REG_INIT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_BASIC_SPEC, bool, O>;
#[doc = "Field `reg_af_en` reader - "]
pub type REG_AF_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_af_en` writer - "]
pub type REG_AF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_BASIC_SPEC, bool, O>;
#[doc = "Field `reg_config_req` reader - "]
pub type REG_CONFIG_REQ_R = crate::BitReader<bool>;
#[doc = "Field `reg_config_req` writer - "]
pub type REG_CONFIG_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHS_BASIC_SPEC, bool, O>;
#[doc = "Field `reg_config_gnt` reader - "]
pub type REG_CONFIG_GNT_R = crate::BitReader<bool>;
#[doc = "Field `reserved_4_7` reader - "]
pub type RESERVED_4_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_mode_reg` reader - "]
pub type REG_MODE_REG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_mode_reg` writer - "]
pub type REG_MODE_REG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_BASIC_SPEC, u8, u8, 8, O>;
#[doc = "Field `reg_addrmb_msk` reader - "]
pub type REG_ADDRMB_MSK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_addrmb_msk` writer - "]
pub type REG_ADDRMB_MSK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_BASIC_SPEC, u8, u8, 8, O>;
#[doc = "Field `reserved_24_27` reader - "]
pub type RESERVED_24_27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_linear_bnd_b` reader - "]
pub type REG_LINEAR_BND_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_linear_bnd_b` writer - "]
pub type REG_LINEAR_BND_B_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UHS_BASIC_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_init_en(&self) -> REG_INIT_EN_R {
        REG_INIT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_af_en(&self) -> REG_AF_EN_R {
        REG_AF_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_config_req(&self) -> REG_CONFIG_REQ_R {
        REG_CONFIG_REQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_config_gnt(&self) -> REG_CONFIG_GNT_R {
        REG_CONFIG_GNT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reserved_4_7(&self) -> RESERVED_4_7_R {
        RESERVED_4_7_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn reg_mode_reg(&self) -> REG_MODE_REG_R {
        REG_MODE_REG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn reg_addrmb_msk(&self) -> REG_ADDRMB_MSK_R {
        REG_ADDRMB_MSK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn reserved_24_27(&self) -> RESERVED_24_27_R {
        RESERVED_24_27_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reg_linear_bnd_b(&self) -> REG_LINEAR_BND_B_R {
        REG_LINEAR_BND_B_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_init_en(&mut self) -> REG_INIT_EN_W<0> {
        REG_INIT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_af_en(&mut self) -> REG_AF_EN_W<1> {
        REG_AF_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_config_req(&mut self) -> REG_CONFIG_REQ_W<2> {
        REG_CONFIG_REQ_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mode_reg(&mut self) -> REG_MODE_REG_W<8> {
        REG_MODE_REG_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_addrmb_msk(&mut self) -> REG_ADDRMB_MSK_W<16> {
        REG_ADDRMB_MSK_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_linear_bnd_b(&mut self) -> REG_LINEAR_BND_B_W<28> {
        REG_LINEAR_BND_B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHS_basic\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhs_basic](index.html) module"]
pub struct UHS_BASIC_SPEC;
impl crate::RegisterSpec for UHS_BASIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhs_basic::R](R) reader structure"]
impl crate::Readable for UHS_BASIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhs_basic::W](W) writer structure"]
impl crate::Writable for UHS_BASIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uhs_basic to value 0xa01f_0000"]
impl crate::Resettable for UHS_BASIC_SPEC {
    const RESET_VALUE: Self::Ux = 0xa01f_0000;
}
